use crate::*;
use std::collections::BTreeMap;

const BASIC: &str = include_str!("basic.yml");

#[test]
fn it_works() {
    let val: Document = serde_yaml::from_str(BASIC).unwrap();
    let map: BTreeMap<Document, Document> = val["somedict"].as_map().unwrap();
    let mut ctx = Document::default();
    ctx["one"]["two"]["three"] = "val".into();
    for (_, v) in map.iter() {
        let res = Templar::global().parse_str(&v.to_string()).unwrap();
        println!("Result: {}", res.render(&ctx).unwrap());
    }
}