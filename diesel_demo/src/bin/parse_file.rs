extern crate diesel_demo;
extern crate diesel;

use std::str;
use std::fs;
use std::path;
use std::collections::HashMap;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
struct Channel {
    title: String,
    name: String,
    url: String,
}

use std::any::type_name;

fn test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}

fn main() {
    println!("read file");
    let file = path::Path::new("./src/feedly.opml");
    println!("{:?}", &file);
    let context = fs::read_to_string(file).expect("!!!!");

    let mut reader = Reader::from_str(&context);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut channelVec: Vec<Channel> = Vec::new();
    // let map = HashMap::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            // Ok(Event::Start(ref e)) => {
            //     match e.name() {
            //         b"outline" => {
            //             let a = e.attributes().map(|a| {
            //                 let attr = a.unwrap();
            //                 (str::from_utf8(attr.key).unwrap(), unsafe { std::str::from_utf8_unchecked(attr.value.as_ref())})
            //             }).collect::<HashMap<_, _>>();

            //             channelVec.push(a);

            //             // for (key, value) in &a {
            //             //     println!("{}: {}", key, unsafe { std::str::from_utf8_unchecked(value.as_ref())});
            //             // }

            //         },
            //         b"tag2" => count += 1,
            //         _ => (),
            //     }
            // },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"outline" => {
                        let attrs = e.attributes().map(|a| {
                            let attr = a.unwrap();
                            (str::from_utf8(attr.key).unwrap(), attr.value)
                        }).collect::<HashMap<_, _>>();

                        // for (key, value) in attrs {
                        //     // println!("{}: {}", key, unsafe { std::str::from_utf8_unchecked(value.as_ref())});
                        //     let mut map = HashMap::new();
                        //     map.insert(key, value);
                        //     // channelVec.push(map);
                        // }
                        println!("{:?}", attrs);
                    },
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    println!("{:?}", channelVec);
    println!("{:?}", txt);
}