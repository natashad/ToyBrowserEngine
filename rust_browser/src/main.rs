use std::io::Read;
use std::fs::File;

pub mod dom;
pub mod html_parser;

fn main() {

    let html = read_source("examples/test.html".to_string());
    let root_node = html_parser::parse(html);
    dom::pretty_print(&root_node);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
