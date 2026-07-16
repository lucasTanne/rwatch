use inotify::EventMask;

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct EventRecord {
    name: String,
    subject: String,
}

pub fn new(subject: String, mask: EventMask) -> Result<EventRecord, ()> {
    let name = match utils::events::event_mask_to_string(mask) {
        Some(n) => n,
        None => return Err(()),
    };

    Ok(EventRecord {
        name: name,
        subject: subject,
    })
}

#[test]
fn test_new() {
    let subject = String::from("test");
    let mask = EventMask::ACCESS;

    let expected = EventRecord {
        name: String::from("ACCESS"),
        subject: String::from("test"),
    };
    let res = new(subject, mask).expect("expecting new event record");
    assert_eq!(expected, res)
}

impl EventRecord {
    pub fn to_string(&self) -> String {
        format!("EVENT [{}] {}", self.name, self.subject)
    }
}

#[test]
fn test_to_string() {
    let event_record = EventRecord {
        name: String::from("test"),
        subject: String::from("ACCESS"),
    };

    let expected = String::from("EVENT [test] ACCESS");
    let res = event_record.to_string();
    assert_eq!(expected, res)
}
