use std::env;
use std::path::Path;

use inotify::{EventMask, Inotify, WatchMask};

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

    let mut buff = [0u8; 4096];
    loop {
        println!("Waiting for event...");

        let events = inotify.read_events_blocking(&mut buff).expect("unable to read events");

        for event in events {
            match event.mask {
                EventMask::MODIFY => {
                    println!("file modified")
                }
                EventMask::OPEN => {
                    println!("file opened")
                }
                EventMask::CLOSE_WRITE => {
                    println!("file closed after written")
                }
                _ => panic!("mask {:?} unsupported yet", event.mask)
            }
        }
    }
}
