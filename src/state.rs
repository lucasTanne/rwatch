use std::sync::Arc;

use tokio::sync::Mutex;

use crate::event_record_list;

pub struct AppState {
    pub event_record_list: Mutex<event_record_list::EventRecordList>
}

pub fn new() -> Arc<AppState> {
    Arc::new(AppState{
        event_record_list: Mutex::new(event_record_list::new())
    })
}