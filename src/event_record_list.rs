use crate::event_record;

pub struct EventRecordList {
    records: Vec<event_record::EventRecord>,
}

pub fn new() -> EventRecordList {
    EventRecordList { records: Vec::new() }
}

impl EventRecordList {
    pub fn push(&mut self, e: event_record::EventRecord) {
        println!("PUSH");
        self.records.push(e);
    }
}
