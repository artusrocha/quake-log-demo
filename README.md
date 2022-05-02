# quake-log-demo

This repository contains a demo written in Rust (with small pieces of js)

It has a implementation in Rust of a quake 3 log parse.

```sh
./
├── Cargo.toml
├── src
    ├── analysis
    │   ├── mod.rs
    │   └── report.rs
    ├── lib.rs
    └── main.rs
└── www
    ├── favicon.ico
    └── index.html
├── server
    ├── index.js
    ├── package.json
    └── package-lock.json
```
The parse logic is implemented in Rust on module `src/analysis`.

The file `src/main.rs` contains the extructure for use in a CLI version, with input from STDIN support and read file (when filename is present)/ as parameter).
  
The file `src/lib.rs` contains the extructure for use in a WebAssembly version for use as wasm ffi (webpage or server side ffi).

## Use as CLI

```sh
cargo build --release
# After this, dthe cli will created at target/release/quake-log-challenge
# execute passing file name parameter:
target/release/quake-log-challenge samples/qgames.log

# OR
# execute passing STDIN:
cat samples/qgames.log | target/release/quake-log-challenge 
```


## Use WASM FFI on webpage

```bash
npx wasm-pack build --target web --out-dir ./www/pkg
# It will compile for wasm on and create binds for js
# run ANY http server on www/ folder
# ex: 
npx http-server www 
# acess web page on your browser (http://127.0.0.1:8080)
```

# Use WASM FFI on node.js server side

```bash

```