extern crate diesel_demo;
extern crate diesel;

use std::fs;
use std::path;
use opml::OPML;

#[derive(Debug)]
struct Channel {
    title: String,
    name: String,
    url: String,
}

fn main() {
    println!("read file");
    let file = path::Path::new("./src/feedly.opml");
    println!("{:?}", &file);
    let context = fs::read_to_string(file).expect("!!!!");
    let document = OPML::from_str(&context).unwrap();
    let outlines = document.body.outlines;

    for outline in &outlines {
        println!("======");
        println!("{:?}", outline);
        println!("======");
        let c = Channel {
            title: outline.title.as_ref(),
            name: outline.text,
            url: outline.xml_url.unwrap(),
        };

        println!("{:?}", c);
    }
}