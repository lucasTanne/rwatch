use std::path::{Path};

use inotify::{EventMask, Inotify, WatchMask};

mod event_record;
mod event_record_list;
mod utils;
mod api;
mod state;
mod cli;

#[tokio::main]
async fn main() {
    let m = cli::create_cli().get_matches();
    let file_path = m.get_one::<String>("watch").map(|f| {
        let fp = Path::new(f);
        if !fp.exists() {
            panic!("Error: {} no such file or directory", fp.display());
        }
        fp.to_path_buf()
    }).unwrap();

    let enable_api = m.get_one::<bool>("enable-api").unwrap();

    let shared_state = state::new();

    let task_state = shared_state.clone();
    tokio::spawn(async move {
        let mut inotify = Inotify::init().expect("unable to initialize inotify");
        let _watch_descriptor = inotify
            .watches()
            .add(file_path.clone(), WatchMask::ALL_EVENTS)
            .expect("failed to watch file");

        utils::logs::log_with_time(format!("start watching {}", file_path.display()));

        let mut buff = [0u8; 4096];

        'outer: loop {
            let events = inotify
                .read_events_blocking(&mut buff)
                .expect("unable to read events");

            for event in events {
                if event.mask == EventMask::IGNORED {
                    utils::logs::log_with_time(String::from("Watch descriptor close, stopping..."));
                    break 'outer;
                }

                let event_record = match event_record::new(file_path.display().to_string(), event.name, event.mask) {
                    Ok(e) => {
                        utils::logs::log(e.to_string());
                        e
                    },
                    Err(_) => {
                        utils::logs::log_with_time(format!("unable to handle event mask: {:?}", event.mask));
                        continue;
                    }
                };

                task_state.event_record_list.lock().await.push(event_record);
            }
        }
    });

    if *enable_api {
        api::start(shared_state).await;
    }
}
