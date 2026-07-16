# Rwatch

Rwatch is a `userland` software which monitor events on an existing file in a Linux operating system.

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
target/release/rwatch {filepath}
```

## How to use

```bash
# Create a dummy file
touch /tmp/a

# From the project
target/release/rwatch /tmp/a
```

Rwatch starts and wait for events.

```bash
# Write into the file
echo "This is a test" > /tmp/a
```

You will receive that output:

```text
Args: ["target/release/rwatch", "/tmp/a"]
start watching /tmp/a
start watching file...
Waiting for event...
EventMask(OPEN)
EventMask(MODIFY)
Waiting for event...
EventMask(MODIFY)
EventMask(CLOSE_WRITE)
Waiting for event...
```

From this output, you can see that the file:
- Were opened
- Were modified
- And closed after being updated

## REST API

The REST API is hard configured to be use from `http://0.0.0.0:3000`.

There are few routes you can use to retrieve event logs:
- `/events` => to list events
- `/events/last` => to retrieve the last events

Here is a example of the `List handler`:

```bash
curl http://127.0.0.1:3000/events
```

Here is the result in `JSON`:

```text
[{"name":"OPEN","subject":"/tmp/a"},{"name":"MODIFY","subject":"/tmp/a"},{"name":"MODIFY","subject":"/tmp/a"},{"name":"CLOSE_WRITE","subject":"/tmp/a"}]
```
## ROADMAP

This project is under development, see ROADMAP.md.
