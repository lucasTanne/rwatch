use inotify::EventMask;

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EventRecord {
    name: String,
    subject: String,
}

pub fn new(subject: String, mask: EventMask) -> Result<EventRecord, ()> {
    let name = match utils::events::event_mask_to_string(mask) {
        Some(n) => n,
        None => return Err(())
    };

    Ok(EventRecord {
        name: name,
        subject: subject,
    })
}

impl EventRecord {
    pub fn to_string(&self) -> String {
        format!("EVENT [{}] {}", self.name, self.subject)
    }
}
