use std::env;
use std::path::Path;

use inotify::{Inotify, WatchMask};

mod event_record;
mod event_record_list;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    if args.len() == 1 || args.len() > 2 {
        panic!("Error: file path required");
    }

    println!("start watching {}", &args[1]);

    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        panic!("Error: {} no such file or directory", file_path.display());
    }

    let mut inotify = Inotify::init().expect("could not initialize inotify");
    let _watch_descriptor = inotify.watches().add(file_path, WatchMask::ALL_EVENTS).expect("failed to watch file");

    println!("start watching file...");

    let mut record_list = event_record_list::new();

    let mut buff = [0u8; 4096];
    loop {
        println!("Waiting for event...");

        let events = inotify.read_events_blocking(&mut buff).expect("unable to read events");
        for event in events {
            
            let event_record = match event_record::new(file_path.display().to_string(), event.mask) {
                Ok(e) => e,
                Err(_) => {
                    println!("Unable to handle event mask: {:?}", event.mask);
                    continue;
                }
            };
            record_list.push(event_record);
        }
    }
}
