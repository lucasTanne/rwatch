use std::{env, sync::Arc};
use std::path::Path;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use inotify::{EventMask, Inotify, WatchMask};

use axum::{Router, routing::get};
use tokio::sync::Mutex;

mod event_record;
mod event_record_list;
mod utils;

struct AppState {
    event_record_list: Mutex<event_record_list::EventRecordList>
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    if args.len() == 1 || args.len() > 2 {
        panic!("Error: file path required");
    }

    let arg_file = args[1].clone();

    let shared_state = Arc::new(AppState{
        event_record_list: Mutex::new(event_record_list::new())
    });


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
            println!("Waiting for event...");

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
                        println!("{:?}", event.mask);
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

    let app = Router::new()
        .route("/events", get(list_events_handler))
        .route("/events/last", get(get_last_event_handler))
        .with_state(shared_state.clone());
    let listener = tokio::net::TcpListener::bind("0000:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_last_event_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.event_record_list.lock().await.get_last_event() {
        Some(e) => Json(e.clone()).into_response(),
        None => StatusCode::NO_CONTENT.into_response()
    }
}

async fn list_events_handler(State(state): State<Arc<AppState>>) -> Json<Vec<event_record::EventRecord>> {
    let guard = state.event_record_list.lock().await;
    Json(guard.list_events().to_vec())
}
