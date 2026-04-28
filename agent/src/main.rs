mod proc;
use crate::proc::{get_process_details, get_process_ids, get_uptime};
use std::fs::{DirEntry, File, ReadDir, read_dir};

fn main() {
    for i in get_process_ids().unwrap() {
        get_process_details(i, 0.0);
    }
}
