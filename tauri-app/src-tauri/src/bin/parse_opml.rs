extern crate app;
extern crate diesel;

use std::fs;
use std::path;
use opml::OPML;

#[derive(Debug)]
struct Channel {
    title: Option<String>,
    name: String,
    url: Option<String>,
}

fn main() {
    println!("read file");
    let file = path::Path::new("./src/feedly.opml");
    println!("{:?}", &file);
    let context = fs::read_to_string(file).expect("!!!!");
    let document = OPML::from_str(&context).unwrap();
    let outlines = document.body.outlines;
    let mut list: Vec<Channel> = Vec::new();

    for outline in outlines {
        let c = Channel {
            title: outline.title,
            name: outline.text,
            url: outline.xml_url,
        };


        println!("{:?}", c);
        list.push(c);
    }
}