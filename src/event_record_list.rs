use crate::event_record;

pub struct EventRecordList {
    records: Vec<event_record::EventRecord>,
}

pub fn new() -> EventRecordList {
    EventRecordList { records: Vec::new() }
}

impl EventRecordList {
    pub fn push(&mut self, e: event_record::EventRecord) {
        self.records.push(e);
    }

    pub fn get_last_event(&self) -> Option<&event_record::EventRecord> {
        self.records.last()
    }
}
