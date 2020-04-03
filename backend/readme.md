# Rust Websocket Server

## Setup

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. `cargo run`

## Interacting

Currently just a websocket echo server.

In your browser console (doesn't matter what website);

```js
var s = new WebSocket("ws://localhost:9999");
s.onmessage = (e) => console.log(e.data);
s.send("echo!");
```

## Open questions

- (High Priority) How to store state of all connections? Some type of tree or hashtable? what should the id be?
  - How do we access this state from all tasks? can we just do that with a global? We may need a mutex or something.
- (Low Priority) How do we enable TLS? Do we terminate that with like nginx or something? or do we need to enable it
  inside of this project and provide a cert here?
  

