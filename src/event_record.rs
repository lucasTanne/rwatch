use std::ffi::OsStr;

use chrono::Local;
use inotify::EventMask;

use crate::utils;
use serde::Serialize;

/*
EventRecord represents an event on a specific subject (file or directory)

Parameters:
- name: Name of the event
- subject: name of the watching file/directory
- target: name of a file/directory which is inside of a watching directory
- is_dir: true if the event is on a directory, can be subject (if target is None) or target
- created_at: timestamp of the event in RFC3339 format
*/
#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct EventRecord {
    name: String,
    subject: String,
    target: Option<String>,
    is_dir: bool,
    created_at: String,
}

pub fn new(
    subject: String,
    event_name: Option<&OsStr>,
    event_mask: EventMask,
) -> Result<EventRecord, ()> {
    let now = Local::now().to_rfc3339();
    let name = match utils::events::event_mask_to_string(&event_mask) {
        Some(n) => n,
        None => return Err(()),
    };
    let target = event_name.map(|n| n.to_str().unwrap_or("incorrect").to_string());

    let is_dir = event_mask.contains(EventMask::ISDIR);

    Ok(EventRecord {
        name: name,
        subject: subject,
        target: target,
        is_dir: is_dir,
        created_at: now,
    })
}

impl EventRecord {
    pub fn to_string(&self) -> String {
        let base = format!("{} {} {}", self.created_at, self.subject, self.name);
        self.target.as_ref().map_or(base.clone(), |t| {
            format!("{} on {}/{}", base, self.subject, t)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_new_correct_event_on_file() {
        let subject = String::from("/tmp/test");

        let expected = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: None,
            is_dir: false,
            created_at: Local::now().to_rfc3339(),
        };
        let res = new(subject, None, EventMask::ACCESS).expect("expecting new event record");
        assert_eq!(expected.name, res.name);
        assert_eq!(expected.subject, res.subject);

        // Test created_at using RFC3339 regex
        let rfc3339_regex = Regex::new(
            r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$",
        )
        .unwrap();
        assert!(rfc3339_regex.is_match(&expected.created_at) == true);
    }

    #[test]
    fn test_new_correct_event_on_directory() {
        let subject = String::from("/tmp/test");

        let expected = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: None,
            is_dir: true,
            created_at: Local::now().to_rfc3339(),
        };
        let res = new(subject, None, EventMask::ACCESS.union(EventMask::ISDIR))
            .expect("expecting new event record");
        assert_eq!(expected.name, res.name);
        assert_eq!(expected.subject, res.subject);

        // Test created_at using RFC3339 regex
        let rfc3339_regex = Regex::new(
            r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$",
        )
        .unwrap();
        assert!(rfc3339_regex.is_match(&expected.created_at) == true);
    }

    #[test]
    fn test_new_correct_event_on_directory_child_file() {
        let subject = String::from("/tmp/test");

        let expected = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: Some(String::from("testfile")),
            is_dir: false,
            created_at: Local::now().to_rfc3339(),
        };
        let res = new(subject, Some(OsStr::new("testfile")), EventMask::ACCESS)
            .expect("expecting new event record");
        assert_eq!(expected.name, res.name);
        assert_eq!(expected.subject, res.subject);

        // Test created_at using RFC3339 regex
        let rfc3339_regex = Regex::new(
            r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$",
        )
        .unwrap();
        assert!(rfc3339_regex.is_match(&expected.created_at) == true);
    }

    #[test]
    fn test_new_correct_event_on_directory_child_directory() {
        let subject = String::from("/tmp/test");

        let expected = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: Some(String::from("testfile")),
            is_dir: true,
            created_at: Local::now().to_rfc3339(),
        };
        let res = new(
            subject,
            Some(OsStr::new("testfile")),
            EventMask::ACCESS.union(EventMask::ISDIR),
        )
        .expect("expecting new event record");
        assert_eq!(expected.name, res.name);
        assert_eq!(expected.subject, res.subject);

        // Test created_at using RFC3339 regex
        let rfc3339_regex = Regex::new(
            r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$",
        )
        .unwrap();
        assert!(rfc3339_regex.is_match(&expected.created_at) == true);
    }

    #[test]
    fn test_to_string_without_target() {
        let now = Local::now().to_rfc3339();
        let event_record = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: None,
            is_dir: false,
            created_at: now.clone(),
        };

        let expected = format!("{} /tmp/test ACCESS", now);
        let res = event_record.to_string();
        assert_eq!(expected, res)
    }

    #[test]
    fn test_to_string_on_directory_without_target() {
        let now = Local::now().to_rfc3339();
        let event_record = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: None,
            is_dir: true,
            created_at: now.clone(),
        };

        let expected = format!("{} /tmp/test ACCESS", now);
        let res = event_record.to_string();
        assert_eq!(expected, res)
    }

    #[test]
    fn test_to_string_with_target() {
        let now = Local::now().to_rfc3339();
        let event_record = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: Some(String::from("testfile")),
            is_dir: false,
            created_at: now.clone(),
        };

        let expected = format!("{} /tmp/test ACCESS on /tmp/test/testfile", now);
        let res = event_record.to_string();
        assert_eq!(expected, res)
    }

    #[test]
    fn test_to_string_on_directory_with_target() {
        let now = Local::now().to_rfc3339();
        let event_record = EventRecord {
            name: String::from("ACCESS"),
            subject: String::from("/tmp/test"),
            target: Some(String::from("testfile")),
            is_dir: true,
            created_at: now.clone(),
        };

        let expected = format!("{} /tmp/test ACCESS on /tmp/test/testfile", now);
        let res = event_record.to_string();
        assert_eq!(expected, res)
    }
}
