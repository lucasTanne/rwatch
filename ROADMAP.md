# Roadmap

## Release 0.1.0

- [x] File path identification
- [x] Use inotify wrapper from crates.io to watch a file and retrieve events
- [x] Store events in a in-memory list
- [x] Write an HTTP API to list events and watch context
- [x] Implement unit tests

## Release 0.2.0

- [ ] Replace println!() by a logging system as well as for events
- [ ] Directory monitoring
- [ ] Stop API when exiting watch loop

## Release 0.3.0

- [ ] CLI context (--enable-api)
- [ ] Add README.md

## Release 0.4.0

- [ ] Watch multiple files/directories

## Release 0.5.0

- [ ] Installation via cargo install

## Ideas

- service systemd
- watchdog systemd
- Support unix socket