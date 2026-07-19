use crate::event_record;

#[derive(Debug, PartialEq)]
pub struct EventRecordList {
    records: Vec<event_record::EventRecord>,
}

pub fn new() -> EventRecordList {
    EventRecordList {
        records: Vec::new(),
    }
}

impl EventRecordList {
    pub fn push(&mut self, e: event_record::EventRecord) {
        self.records.push(e);
    }

    pub fn list_events(&self) -> &[event_record::EventRecord] {
        &self.records
    }

    pub fn get_last_event(&self) -> Option<&event_record::EventRecord> {
        self.records.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inotify::EventMask;

    #[test]
    fn test_new() {
        let expected = EventRecordList {
            records: Vec::new(),
        };
        let res = new();
        assert_eq!(expected, res);
    }

    #[test]
    fn test_push() {
        let mut list = new();
        let event_record = event_record::new(String::from("/tmp/test"), None, EventMask::ACCESS)
            .expect("expected new event record");

        let expected = vec![event_record.clone()];
        list.push(event_record);
        assert!(list.records.len() == 1);
        assert_eq!(expected, list.records);
    }

    #[test]
    fn test_list_events() {
        let mut list = new();
        let event_record = event_record::new(String::from("/tmp/test"), None, EventMask::ACCESS)
            .expect("expected new event record");

        let expected = &[event_record.clone()];
        list.push(event_record);

        let res = list.list_events();
        assert!(res.len() == 1);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_get_last_event() {
        let mut list = new();
        let event_record1 = event_record::new(String::from("/tmp/test"), None, EventMask::ACCESS)
            .expect("expected new event record access");
        list.push(event_record1.clone());

        let mut res = list.get_last_event().expect("expected access event");
        assert_eq!(&event_record1, res);

        let event_record2 = event_record::new(String::from("/tmp/test"), None, EventMask::MODIFY)
            .expect("expected new event record modify");
        list.push(event_record2.clone());

        res = list.get_last_event().expect("expected modify event");
        assert_eq!(&event_record2, res);
    }
}
