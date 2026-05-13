use shared::process::ProcessSnapShot;
use shared::{Process, Processes, SnapShotCollector};
use std::error::Error;
use std::fs::{DirEntry, File, ReadDir, read_dir};
use std::io::Read;
use std::path::PathBuf;

const PROC_PATH: &str = "/proc";

pub fn collect_processes_data(
    collector: &mut SnapShotCollector,
    cores: u64
) -> Result<Processes, Box<dyn Error>> {
    let machine_id = get_machine_id()?;
    let uptime = get_uptime()?;
    let mut processes = Processes::new(machine_id);
    get_process_ids()?
        .into_iter()
        .filter_map(|x| get_process_details(x, uptime, collector, cores).ok())
        .for_each(|x| processes.add_process(x));
    processes.finalize();

    Ok(processes)
}

fn get_process_ids() -> Result<Vec<u64>, Box<dyn Error>> {
    let dir = read_dir(PROC_PATH)?;
    Ok(dir
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.file_name().to_str().unwrap().parse())
        .filter_map(|x| x.ok())
        .collect())
}

pub fn get_process_details(
    id: u64,
    uptime: f64,
    collector: &mut SnapShotCollector,
    cores: u64
) -> Result<Process, Box<dyn Error>> {
    let base = PathBuf::from(PROC_PATH).join(id.to_string());
    let mut status_file = File::open(base.join("status"))?;
    let mut stat_file = File::open(base.join("stat"))?;
    let mut smaps_rollup_file = File::open(base.join("smaps_rollup"))?;
    let (name, threads) = get_name(&mut status_file)?;
    let (ultime, stime, start_time) = get_ultime_and_time(&mut stat_file)?;
    let avg_cpu_usage = Process::calculate_avg_cpu_usage(ultime, stime, start_time, uptime, cores);
    let (rss, pss) = get_rss_and_pss(&mut smaps_rollup_file)?;
    let snapshot = ProcessSnapShot {
        ultime,
        stime,
        uptime,
    };
    let instant_cpu_usage = Process::calculate_instant_cpu_usage(&snapshot, &collector.get(id), cores);
    collector.add(id, snapshot);
    Ok(Process::new(
        id,
        name,
        rss,
        pss,
        avg_cpu_usage,
        instant_cpu_usage,
        threads,
    ))
}

fn get_rss_and_pss(smaps_roll_up_file: &mut File) -> Result<(u64, u64), Box<dyn Error>> {
    let mut contents = String::from("");
    smaps_roll_up_file.read_to_string(&mut contents)?;
    let contents = contents.lines().collect::<Vec<_>>();
    let rss = contents
        .iter()
        .find(|x| x.starts_with("Rss:"))
        .and_then(|x| x[4..].replace(" kB", "").trim().parse().ok())
        .unwrap_or(0);
    let pss = contents
        .iter()
        .find(|x| x.starts_with("Pss:"))
        .and_then(|x| x[4..].replace(" kB", "").trim().parse().ok())
        .unwrap_or(0);
    Ok((rss, pss))
}

fn get_ultime_and_time(stat_file: &mut File) -> Result<(f64, f64, f64), Box<dyn Error>> {
    let mut contents = String::from("");
    stat_file.read_to_string(&mut contents)?;
    let contents = contents.split_whitespace().collect::<Vec<_>>();
    let ultime = contents.get(13).unwrap().parse()?;
    let stime = contents.get(14).unwrap().parse()?;
    let start_time = contents.get(21).unwrap().parse()?;
    Ok((ultime, stime, start_time))
}

fn get_name(status_file: &mut File) -> Result<(String, u64), Box<dyn Error>> {
    let mut content = String::from("");
    status_file.read_to_string(&mut content)?;
    let content = content.lines().collect::<Vec<_>>();
    let name = &content.get(0).unwrap_or(&"Name: Unknown")[6..];
    let threads = content
        .iter()
        .find(|x| x.starts_with("Threads:"))
        .and_then(|x| x[8..].trim().parse().ok())
        .unwrap_or(1);
    Ok((name.to_string(), threads))
}

fn get_uptime() -> Result<f64, Box<dyn Error>> {
    let mut uptime_string = String::from("");
    let mut uptime_file = File::open("/proc/uptime")?;
    uptime_file.read_to_string(&mut uptime_string)?;
    let uptime = uptime_string.split_whitespace().collect::<Vec<_>>();
    let uptime = uptime.get(0).ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to find the second field in the uptime file",
        ))
    })?;
    match uptime.parse() {
        Err(e) => Err(Box::new(e)),
        Ok(v) => Ok(v),
    }
}

fn get_machine_id() -> Result<String, Box<dyn Error>> {
    let id = std::fs::read_to_string("/etc/machine-id")?;
    Ok(id)
}
