extern crate atom_syndication;

use atom_syndication::{Feed, Link};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn link_is_alternate(link: &Link) -> bool {
    link.rel == Some("alternate".to_string())
}

fn main() {
    let mut file = File::open("test-data/xkcd.xml").unwrap();
    let mut atom_string = String::new();
    file.read_to_string(&mut atom_string).unwrap();
    let feed = Feed::from_str(&atom_string).unwrap();

    for entry in feed.entries {
        println!("{}: {}", entry.title, entry.links.into_iter().find(link_is_alternate).unwrap().href);
    }
}
