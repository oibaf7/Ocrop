use shared::Process;
use std::error::Error;
use std::fmt::{Error, format};
use std::fs::{DirEntry, File, ReadDir, read_dir};
use std::io::{ErrorKind, Read};
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

pub fn get_process_details(id: u64) -> Result<Process, Box<dyn Error>> {
    let base = PathBuf::from(PROC_PATH).join(id.to_string());
    let status_file = File::open(base.join("status"))?;
    let stat_file = File::open(base.join("stat"))?;
    let smaps_rollup_file = File::open(base.join("smaps_rollup"))?;
    let mut uptime_file = File::open("/proc/uptime")?;

    let uptime: f64 = get_uptime(&mut uptime_file)?;

    Ok(Process::new(0, String::from(""), 0, 0, 0.0, 0))
}

fn get_uptime(uptime_file: &mut File) -> Result<f64, Box<dyn Error>> {
    let mut uptime_string = String::from("");
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
