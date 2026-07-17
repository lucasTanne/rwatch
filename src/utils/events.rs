use inotify::EventMask;

/// returns the given event mask in string
/// 
/// If the mask contains a modifier bit like ISDIR, it will be removed
///
/// See https://man7.org/linux/man-pages/man7/inotify.7.html
pub fn event_mask_to_string(mask: &EventMask) -> Option<String> {
    let m = mask.difference(EventMask::ISDIR);
    match m {
        // Modify file's content
        EventMask::MODIFY => Some(String::from("MODIFY")),
        // Open file
        EventMask::OPEN => Some(String::from("OPEN")),
        // Access to file's content
        EventMask::ACCESS => Some(String::from("ACCESS")),
        // Close file after writting
        EventMask::CLOSE_WRITE => Some(String::from("CLOSE_WRITE")),
        // Close the file without writting
        EventMask::CLOSE_NOWRITE => Some(String::from("CLOSE_NOWRITE")),
        // Change file's metadata
        EventMask::ATTRIB => Some(String::from("ATTRIB")),
        // file was deleted
        EventMask::DELETE_SELF => Some(String::from("DELETE_SELF")),
        // watching file's watch descriptor doesn't exists anymore, there won't be any events
        // e.g file removed
        EventMask::IGNORED => Some(String::from("IGNORED")),
        _ => None,
    }
}

#[test]
fn test_event_mask_to_string() {
    struct Test {
        mask: EventMask,
        expected: String,
        should_fail: bool,
    }

    let tests = &[
        Test {
            mask: EventMask::MODIFY,
            expected: String::from("MODIFY"),
            should_fail: false,
        },
        Test {
            mask: EventMask::MODIFY.union(EventMask::ISDIR),
            expected: String::from("MODIFY"),
            should_fail: false,
        },
        Test {
            mask: EventMask::OPEN,
            expected: String::from("OPEN"),
            should_fail: false,
        },
        Test {
            mask: EventMask::OPEN.union(EventMask::ISDIR),
            expected: String::from("OPEN"),
            should_fail: false,
        },
        Test {
            mask: EventMask::ACCESS,
            expected: String::from("ACCESS"),
            should_fail: false,
        },
        Test {
            mask: EventMask::ACCESS.union(EventMask::ISDIR),
            expected: String::from("ACCESS"),
            should_fail: false,
        },
        Test {
            mask: EventMask::CLOSE_WRITE,
            expected: String::from("CLOSE_WRITE"),
            should_fail: false,
        },
        Test {
            mask: EventMask::CLOSE_WRITE.union(EventMask::ISDIR),
            expected: String::from("CLOSE_WRITE"),
            should_fail: false,
        },
        Test {
            mask: EventMask::CLOSE_NOWRITE,
            expected: String::from("CLOSE_NOWRITE"),
            should_fail: false,
        },
        Test {
            mask: EventMask::CLOSE_NOWRITE.union(EventMask::ISDIR),
            expected: String::from("CLOSE_NOWRITE"),
            should_fail: false,
        },
        Test {
            mask: EventMask::ATTRIB,
            expected: String::from("ATTRIB"),
            should_fail: false,
        },
        Test {
            mask: EventMask::ATTRIB.union(EventMask::ISDIR),
            expected: String::from("ATTRIB"),
            should_fail: false,
        },
        Test {
            mask: EventMask::DELETE_SELF,
            expected: String::from("DELETE_SELF"),
            should_fail: false,
        },
        Test {
            mask: EventMask::DELETE_SELF.union(EventMask::ISDIR),
            expected: String::from("DELETE_SELF"),
            should_fail: false,
        },
        Test {
            mask: EventMask::IGNORED,
            expected: String::from("IGNORED"),
            should_fail: false,
        },
        Test {
            mask: EventMask::IGNORED.union(EventMask::ISDIR),
            expected: String::from("IGNORED"),
            should_fail: false,
        },
        // This case is not managed yet
        Test {
            mask: EventMask::CREATE,
            expected: String::from("CREATE"),
            should_fail: true,
        },
    ];

    for t in tests {
        let res = event_mask_to_string(&t.mask);
        if !t.should_fail {
            let v = res.unwrap();
            assert_eq!(t.expected, v);
        } else {
            if !res.is_none() {
                panic!("expected None");
            }
        }
    }
}
