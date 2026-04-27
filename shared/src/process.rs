pub struct Process {
    pid: u32, //from folder name
    name: String, //from the status file
    rss: u64, //in Kb
    pss: u64, //in Kb
    cpu: f64,
    threads: u64
}

const CLK_TCK: u64 = 100;

impl Process {
    pub fn new(pid: u32, name: String, rss: u64, pss: u64, cpu: f64, threads: u64) -> Process {
        Process { pid, name, rss, pss, cpu, threads }
    }
    pub fn calculate_cpu_usage(ultime: u64, stime: u64, start_time: u64, uptime: u64) -> f64 {
        let usage: f64 = (ultime as f64 + stime as f64) / CLK_TCK as f64;
        let total_time = (start_time as f64 / CLK_TCK as f64) - uptime as f64;
        usage / total_time
    }
}