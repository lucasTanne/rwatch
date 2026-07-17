use std::env;
use std::path::Path;

use inotify::{EventMask, Inotify, WatchMask};

mod event_record;
mod event_record_list;
mod utils;
mod api;
mod state;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        panic!("Error: file path required");
    }

    let arg_file = args[1].clone();

    let shared_state = state::new();

    let task_state = shared_state.clone();
    tokio::spawn(async move {
        println!("start watching {}", arg_file);

        let file_path = Path::new(&arg_file);
        if !file_path.exists() {
            panic!("Error: {} no such file or directory", file_path.display());
        }

        let mut inotify = Inotify::init().expect("unable to initialize inotify");
        let _watch_descriptor = inotify
            .watches()
            .add(file_path, WatchMask::ALL_EVENTS)
            .expect("failed to watch file");

        println!("start watching file...");

        let mut buff = [0u8; 4096];

        'outer: loop {
            let events = inotify
                .read_events_blocking(&mut buff)
                .expect("unable to read events");

            for event in events {
                if event.mask == EventMask::IGNORED {
                    println!("Watch descriptor close, stopping...");
                    break 'outer;
                }

                let event_record = match event_record::new(file_path.display().to_string(), event.mask) {
                    Ok(e) => {
                        println!("{:?}", e.to_string());
                        e
                    },
                    Err(_) => {
                        println!("unable to handle event mask: {:?}", event.mask);
                        continue;
                    }
                };

                task_state.event_record_list.lock().await.push(event_record);
            }
        }
    });

    api::start(shared_state).await;
}
