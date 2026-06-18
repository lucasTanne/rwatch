use std::sync::Arc;

use axum::{Router, routing::get};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::{event_record, state};

pub async fn start(state: Arc<state::AppState>) {
    let app = Router::new()
        .route("/events", get(list_events_handler))
        .route("/events/last", get(get_last_event_handler))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind("0000:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_last_event_handler(State(state): State<Arc<state::AppState>>) -> impl IntoResponse {
    match state.event_record_list.lock().await.get_last_event() {
        Some(e) => Json(e.clone()).into_response(),
        None => StatusCode::NO_CONTENT.into_response()
    }
}

async fn list_events_handler(State(state): State<Arc<state::AppState>>) -> Json<Vec<event_record::EventRecord>> {
    let guard = state.event_record_list.lock().await;
    Json(guard.list_events().to_vec())
}