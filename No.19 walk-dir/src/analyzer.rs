use std::{fs};
use std::fs::metadata;
use std::path::PathBuf;
use std::ffi::OsStr;

pub fn analyze () {
    println!("start analyze.");
    read_dir();
}

fn read_dir() {
    let root = PathBuf::from("../");
    let mut list = Vec::new();
    
    walk_dir(&root, &mut list);
}

pub fn walk_dir (dir: &PathBuf, list: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir);
    let entries = match entries {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    for entry in entries {
        let _path = entry.unwrap().path();
        let meta = match metadata(&_path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        
        if meta.is_dir() {
            walk_dir(&_path, list)
        }

        if meta.is_file() && _path.extension().and_then(OsStr::to_str) == Some("toml") {
            println!("{:?}", &_path);
            list.push(_path)
        }

    }
}
