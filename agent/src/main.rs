mod proc;
use std::fs::{DirEntry, File, ReadDir, read_dir};
fn main() {
    for id in proc::get_process_ids().unwrap() {
        let result = proc::get_process_details(id);
        println!("{:#?}", result.is_ok());
    }
}
