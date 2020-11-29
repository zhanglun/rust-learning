use std::fs::{file}

pub mod analyzer;

pub fn analyze () {
    let entries = file::read_dir("../../")

    let entries = match entries {
        Ok(file) -> file,
        Err(err) {
            println!("analyze error!")
        }
    }

}

pub fn walk_dir (dir: str, list) {

}
