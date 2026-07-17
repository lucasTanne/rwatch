use inotify::EventMask;
use chrono::{Local};
use regex::Regex;

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct EventRecord {
    name: String,
    subject: String,
    created_at: String,
}

pub fn new(subject: String, mask: EventMask) -> Result<EventRecord, ()> {
    let now = Local::now().to_rfc3339();
    let name = match utils::events::event_mask_to_string(mask) {
        Some(n) => n,
        None => return Err(()),
    };

    Ok(EventRecord {
        name: name,
        subject: subject,
        created_at: now,
    })
}

#[test]
fn test_new() {
    let subject = String::from("test");
    let mask = EventMask::ACCESS;
    
    let expected = EventRecord {
        name: String::from("ACCESS"),
        subject: String::from("test"),
        created_at: Local::now().to_rfc3339(),
    };
    let res = new(subject, mask).expect("expecting new event record");
    assert_eq!(expected.name, res.name);
    assert_eq!(expected.subject, res.subject);

    // Test created_at using RFC3339 regex
    let rfc3339_regex = Regex::new(r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$").unwrap();
    assert!(rfc3339_regex.is_match(&expected.created_at) == true);
}

impl EventRecord {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.created_at, self.subject, self.name)
    }
}

#[test]
fn test_to_string() {
    let now = Local::now().to_rfc3339();
    let event_record = EventRecord {
        name: String::from("test"),
        subject: String::from("ACCESS"),
        created_at: now.clone(),
    };

    let expected = format!("{} ACCESS test", now);
    let res = event_record.to_string();
    assert_eq!(expected, res)
}
