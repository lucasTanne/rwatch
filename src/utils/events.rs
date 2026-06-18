use inotify::EventMask;

/// returns the given event mask in string
///
/// See https://man7.org/linux/man-pages/man7/inotify.7.html
pub fn event_mask_to_string(mask: EventMask) -> Option<String> {
    match mask {
        // Modify file's content
        EventMask::MODIFY => Some(String::from("MODIFY")),
        // Open file
        EventMask::OPEN => Some(String::from("OPEN")),
        // Access to file's content
        EventMask::ACCESS => Some(String::from("ACCESS")),
        // Close file after writting
        EventMask::CLOSE_WRITE => Some(String::from("WRITE")),
        // Close the file without writting
        EventMask::CLOSE_NOWRITE => Some(String::from("CLOSE_NOWRITE")),
        // Change file's metadata
        EventMask::ATTRIB => Some(String::from("ATTRIB")),
        // file was deleted
        EventMask::DELETE_SELF => Some(String::from("DELETE_SELF")),
        // watching file's watch descriptor doesn't exists anymore, there won't be any events
        // e.g file removed
        EventMask::IGNORED => Some(String::from("IGNORED")),
        _ => None
    }
}