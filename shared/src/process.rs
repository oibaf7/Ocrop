use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Processes {
    id: String,
    time_stamp: SystemTime,
    processes: Vec<Process>,
    total_threads: u64,
    total_cpu: f64,
    total_pss: u64,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Process {
    pid: u64,     //from folder name
    name: String, //from the status file
    rss: u64,     //in Kb
    pss: u64,     //in Kb
    cpu_usage: f64,
    threads: u64,
}

const CLK_TCK: u64 = 100;

impl Process {
    pub fn new(pid: u64, name: String, rss: u64, pss: u64, cpu_usage: f64, threads: u64) -> Self {
        Self {
            pid,
            name,
            rss,
            pss,
            cpu_usage,
            threads,
        }
    }
    pub fn calculate_cpu_usage(ultime: f64, stime: f64, start_time: f64, uptime: f64) -> f64 {
        let usage: f64 = (ultime + stime) / CLK_TCK as f64;
        let total_time = uptime - (start_time / CLK_TCK as f64);
        usage / total_time
    }
}

impl Processes {
    pub fn new(id: String) -> Self {
        Self {
            id,
            time_stamp: SystemTime::now(),
            processes: Vec::new(),
            total_threads: 0,
            total_pss: 0,
            total_cpu: 0.0,
        }
    }

    pub fn finalize(&mut self) {
        self.total_threads = self.processes.iter().map(|x| x.threads).sum();
        self.total_pss = self.processes.iter().map(|x| x.pss).sum();
        self.total_cpu = self.processes.iter().map(|x| x.cpu_usage).sum();
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process)
    }
}