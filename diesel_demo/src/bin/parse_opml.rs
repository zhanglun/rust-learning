extern crate diesel_demo;
extern crate diesel;

use std::str;
use std::fs;
use std::path;
use opml::OPML;

fn main() {
    println!("read file");
    let file = path::Path::new("./src/feedly.opml");
    println!("{:?}", &file);
    let context = fs::read_to_string(file).expect("!!!!");
    let document = OPML::from_str(&context).unwrap();

    document.body.outlines.iter().map(|outline| {
        println!("======");
        println!("{:?}", outline);
        println!("======");
    });
    
}