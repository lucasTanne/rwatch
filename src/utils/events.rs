use inotify::EventMask;

pub fn event_mask_to_string(mask: EventMask) -> Option<String> {
    match mask {
        EventMask::MODIFY => Some(String::from("MODIFY")),
        EventMask::OPEN => Some(String::from("OPEN")),
        EventMask::CLOSE_WRITE => Some(String::from("WRITE")),
        _ => None
    }
}