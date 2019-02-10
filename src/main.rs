use std::io;
use std::fs::{self, DirEntry};

// structs for storing the information we need

#[allow(dead_code)]
struct UrlStatus {
    domain : String,
    content_hash : String
}

#[allow(dead_code)]
struct Subscriber {
    domain: String,
    email: String
}

fn parse_url_statuses(folder: &str) {
    let s = UrlStatus {
        domain: "test.com".to_string(),
        content_hash: "abc0101".to_string()
    };

    let mut vec = Vec::new();
    vec.push(s);

    let mut buffer = String::new();

    println!("Looking at {}!", folder);
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    println!("Meta: {:?}", metadata.is_file());
                    if metadata.is_file() {
                        println!("File found: {:?}", entry);
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            println!("Read file!!!");
                            print!("Content: ");
                            println!("{}", content);
                        }
                    }
                }
            }
        }
    }
}

// fn parse_subscribers(folder: &str) {
    
// }

fn main() {
    // But could we iterate over that folder and produce some meaning out of it?
    let path : &'static str = "/home/teodorlu/workspace/bash/riddler/urlstatus";
    parse_url_statuses(path);
    println!("Hello, world!");
}
