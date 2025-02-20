use quick_xml::de::from_str;
use quick_xml::{events::Event, reader::Reader};
// use serde::Deserialize;

pub mod client;

pub struct DAVResponsePropItem {
  __prefix: String,
  __text: String,
}

pub struct DAVResponsePropStat {
  prop: DAVResponseProp,
  status: DAVResponsePropItem,
}

pub struct DAVResponse {
  href: DAVResponseHref,
  prop_stat: DAVResponsePropStat,
  __prefix: String,
}

pub struct MultiStatus {
  response: Vec<DAVResponse>
}

pub fn parse_xml(xml: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    // let mut json_obj = json!({});

    let mut buf = Vec::new();
    let mut key = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(element)) => {
                println!("element Start {:?}", element.name());
                println!("element Start {:?}", element.local_name());
                // key = element.name().to_owned();
                // json_obj[key.clone()] = json!({});
            }
            Ok(Event::Text(element)) => {
                println!(
                    "element Text {:?}",
                    element.unescape().unwrap().into_owned()
                );
                // let value = element.unescape_and_decode(&reader)?;
                // json_obj[key.clone()] = json!(value);
            }
            Ok(Event::End(_)) => {
                key.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }

        buf.clear();
    }

    let data: MultiStatus = from_str(xml)?;

    println!("data {:?},", data);

    Ok(())
}
