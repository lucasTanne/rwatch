# Roadmap

## Release 0.1.0

### Features

- [ ] File path identification
- [ ] Use inotify wrapper from crates.io to watch a file a retrieve events
- [ ] Store events in a in-memory list
- [ ] Write an HTTP API to list events and watch context

## Release 0.2.0

- [ ] Replace println!() by a logging system as well as for events
- [ ] Directory monitoring

## Ideas

- service systemd
- watchdog systemd
- Support unix socket