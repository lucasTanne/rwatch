# Rwatch

Rwatch is a `userland` CLI used to monitor events on an existing file/directory in a Linux operating system.

Events are saved `in-memory only` and are showed in `stdout` and by its `REST API`.

## How it works

Rwatch use `inotify` to watch the targeted file and receive events.

## How to build

Clone it into your machine

```bash
apt install libinotifytools0 libinotifytools0-dev

# From rwatch project
cargo build --release
```

### How to run

```bash
target/release/rwatch --watch {filepath}
```

## How to use

```bash
# Create a dummy file
touch /tmp/a

# From the project
target/release/rwatch --watch /tmp/a
```

Rwatch starts and wait for events.

```bash
# Write into the file
echo "This is a test" > /tmp/a
```

You will receive that output:

```text
2026-07-17T20:01:48.477023579+02:00 start watching /tmp/a
2026-07-17T20:03:24.845420854+02:00 /tmp/a OPEN
2026-07-17T20:03:24.845480045+02:00 /tmp/a MODIFY
2026-07-17T20:03:24.845486714+02:00 /tmp/a CLOSE_WRITE
```

From this output, you can see that the file:
- Were opened
- Were modified
- And closed after being updated

Use the helper flag `-h | --help` to show the helper.

## REST API

The REST API is hard configured to be use from `http://0.0.0.0:3000` and is enabled using the flag `--enable-api`.

There are few routes you can use to retrieve event logs:
- `/events` => to list events
- `/events/last` => to retrieve the last events

Here is a example of the `List handler`:

```bash
target/release/rwatch --watch /tmp/a --enable-api
curl http://127.0.0.1:3000/events
```

Here is the result in `JSON`:

```text
[
    {
        "name": "OPEN",
        "subject": "/tmp/a",
        "created_at": "2026-07-17T19:24:56.452130624+02:00"
    },
    {
        "name": "MODIFY",
        "subject": "/tmp/a",
        "created_at": "2026-07-17T19:24:56.452237169+02:00"
    },
    {
        "name": "MODIFY",
        "subject": "/tmp/a",
        "created_at": "2026-07-17T19:24:56.452258783+02:00"
    },
    {
        "name": "CLOSE_WRITE",
        "subject": "/tmp/a",
        "created_at": "2026-07-17T19:24:56.452269701+02:00"
    }
]
```
## ROADMAP

This project is under development, see ROADMAP.md.
