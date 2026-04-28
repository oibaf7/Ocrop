use shared::Process;
use std::error::Error;
use std::fmt::format;
use std::fs::{DirEntry, File, ReadDir, read_dir};
use std::io::Read;
use std::os::linux::raw::stat;
use std::os::unix::raw::uid_t;
use std::path::PathBuf;

const PROC_PATH: &str = "/proc";

pub fn get_process_ids() -> Result<Vec<u64>, Box<dyn Error>> {
    let dir = read_dir(PROC_PATH)?;
    Ok(dir
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.file_name().to_str().unwrap().parse())
        .filter_map(|x| x.ok())
        .collect())
}

pub fn get_process_details(id: u64, uptime: f64) -> Result<Process, Box<dyn Error>> {
    let base = PathBuf::from(PROC_PATH).join(id.to_string());
    let mut status_file = File::open(base.join("status"))?;
    let mut stat_file = File::open(base.join("stat"))?;
    let mut smaps_rollup_file = File::open(base.join("smaps_rollup"))?;
    let name = get_name(&mut status_file)?;
    let (ultime, stime, start_time) = get_ultime_and_time(&mut stat_file)?;
    let cpu_usage = Process::calculate_cpu_usage(ultime, stime, start_time, uptime);
    
    Ok(Process::new(id, name, 0, 0, cpu_usage, 0))
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

fn get_name(status_file: &mut File) -> Result<String, Box<dyn Error>> {
    let mut content = String::from("");
    status_file.read_to_string(&mut content)?;
    let name = content.lines().collect::<Vec<_>>();
    let name = &name.get(0).unwrap_or(&"Name: Unknown")[6..];
    Ok(name.to_string())
}

pub fn get_uptime() -> Result<f64, Box<dyn Error>> {
    let mut uptime_string = String::from("");
    let mut uptime_file = File::open("/proc/uptime")?;
    uptime_file.read_to_string(&mut uptime_string)?;
    let uptime = uptime_string.split_whitespace().collect::<Vec<_>>();
    let uptime = uptime.get(1).ok_or_else(|| {
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
