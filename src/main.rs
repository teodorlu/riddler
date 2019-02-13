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

/* I should be able to wrap this.
 *
 * std::io types generally return std::io::Result where type Result<T> =
 * Result<T, Error> and std::io::Error is the "error type for most IO errors".
 * This means that we can collapse most IO errors into one, and use monadic
 * composition.
 */

fn parse_url_statuses(folder: &str) {
    let s = UrlStatus {
        domain: "test.com".to_string(),
        content_hash: "abc0101".to_string()
    };

    let mut vec = vec![s];

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

fn parse2(folder: &str) {
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries {
            let r = entry
                .and_then(|e| {
                    Ok(e)
                }).and_then(|e| {
                    println!("Got this far: {:?}", e);
                    Ok(e)
                });
            println!("Finally: {:?}", r);
        }
    }

}

// fn parse_subscribers(folder: &str) {
    
// }

fn main() {
    // But could we iterate over that folder and produce some meaning out of it?
    let path : &'static str = "/home/teodorlu/workspace/bash/riddler/urlstatus";
    // parse_url_statuses(path);
    parse2(path);
    println!("Hello, world!");
}
