use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Processes {
    pub id: String,
    pub processes: Vec<Process>,
    pub total_threads: u64,
    pub total_avg_cpu: f64,
    pub total_instant_cpu: f64,
    pub total_pss: u64,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Process {
    pub pid: u64,     //from folder name
    pub name: String, //from the status file
    pub rss: u64,     //in Kb
    pub pss: u64,     //in Kb
    pub avg_cpu_usage: f64,
    pub instant_cpu_usage: f64,
    pub threads: u64,
}

pub struct ProcessSnapShot {
    pub ultime: f64,
    pub stime: f64,
    pub uptime: f64,
}

pub struct SnapShotCollector {
    map: HashMap<u64, ProcessSnapShot>,
}

const CLK_TCK: u64 = 100;

impl Process {
    pub fn new(pid: u64, name: String, rss: u64, pss: u64, avg_cpu_usage: f64, instant_cpu_usage: f64, threads: u64) -> Self {
        Self {
            pid,
            name,
            rss,
            pss,
            avg_cpu_usage,
            instant_cpu_usage,
            threads,
        }
    }
    pub fn calculate_avg_cpu_usage(ultime: f64, stime: f64, start_time: f64, uptime: f64, cores: u64) -> f64 {
        let usage: f64 = (ultime + stime) / CLK_TCK as f64;
        let total_time = uptime - (start_time / CLK_TCK as f64);
        (usage / total_time * 100.0) / cores as f64
    }

    pub fn calculate_instant_cpu_usage(curr : &ProcessSnapShot, prev : &ProcessSnapShot, cores: u64) -> f64 {
        let delta_cpu = (curr.ultime + curr.stime) - (prev.ultime + prev.stime);
        let delta_time = curr.uptime - prev.uptime; // Δuptime = elapsed real time since start time is fixed

        if delta_time <= 0.0 {
            return 0.0;
        }

        ((delta_cpu / CLK_TCK as f64) / delta_time * 100.0) / cores as f64
    }
}

impl Processes {
    pub fn new(id: String) -> Self {
        Self {
            id,
            processes: Vec::new(),
            total_threads: 0,
            total_pss: 0,
            total_avg_cpu: 0.0,
            total_instant_cpu: 0.0
        }
    }

    pub fn finalize(&mut self) {
        self.total_threads = self.processes.iter().map(|x| x.threads).sum();
        self.total_pss = self.processes.iter().map(|x| x.pss).sum();
        self.total_avg_cpu = self.processes.iter().map(|x| x.avg_cpu_usage).sum();
        self.total_instant_cpu = self.processes.iter().map(|x| x.instant_cpu_usage).sum();
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process)
    }
}

impl SnapShotCollector {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, pid: u64, snapshot: ProcessSnapShot) {
        self.map.insert(pid, snapshot);
    }

    pub fn get(&mut self, pid: u64) -> ProcessSnapShot {
        self.map.remove(&pid).unwrap_or_default()
    }

}

impl Default for ProcessSnapShot {
    fn default() -> Self {
        Self {
            ultime: 0.0,
            stime: 0.0,
            uptime: 0.0,
        }
    }
}

