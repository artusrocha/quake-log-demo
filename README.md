# quake-log-demo [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/artusrocha/quake-log-demo)

This repository contains a demo written in Rust (with small pieces of js)

It has a implementation in Rust of a quake 3 log parse.

- [Use as CLI](#use-as-cli)
- [Use WASM FFI on webpage](#use-wasm-ffi-on-webpage)
- [Use WASM FFI on node.js server side](#use-wasm-ffi-on-nodejs-server-side)


#### Install and setup rust dev tools:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable && rustup update
```

#### Cloning and testing
```bash 
git clone https://github.com/artusrocha/quake-log-demo.git
cd quake-log-demo
cargo test
```

Content of this project
```
./
├── Cargo.toml
├── README.md
├── samples
│   ├── qgames.log
│   └── qgames-small.log
├── server
│   ├── index.js
│   ├── package.json
│   └── package-lock.json
├── src
│   ├── analysis
│   │   ├── mod.rs
│   │   └── report
│   │       ├── death_types.rs
│   │       ├── mod.rs
│   │       └── test.rs
│   ├── lib.rs
│   └── main.rs
└── www
    ├── favicon.ico
    └── index.html
```
The parser logic is implemented in Rust on module `src/analysis`.

The file `src/main.rs` contains the extructure for use in a CLI version, with input from STDIN support and read file (when filename is present)/ as parameter).

The file `src/lib.rs` contains the extructure for use in a WebAssembly version for use as wasm ffi (webpage or server side ffi).

Obs:
- `www` folder contains just files for wasm webpage version.
- `server` folder contains just files for wasm + nodejs server version.

### About tests
Diferent from other languages, in Rust, write tests in the same file of the code we are testing is a *idiomatic* way. Other way is write at other file in the as a submodule and reference it on the file that is been tested. Both ways are exemplified on this project.

## Use as CLI

```bash
cargo build --release
## After this, the cli will be created at target/release/quake-log-demo

## execute passing file name parameter:
target/release/quake-log-demo samples/qgames.log

## OR
## execute passing STDIN:
cat samples/qgames.log | target/release/quake-log-demo
```

demo:

[![asciicast](https://asciinema.org/a/8M6VNnw8fqtxOK1VqQOd1T6eF.svg)](https://asciinema.org/a/8M6VNnw8fqtxOK1VqQOd1T6eF)

## Use WASM FFI on webpage
> You can find a preview -> [here](https://quake-log-demo.pages.dev/)

```bash
npx wasm-pack build --target web --out-dir ./www/pkg
## It will compile for wasm on www/pkg/ folder and create binds for js
## run ANY http server on www/ folder
## ex:
npx http-server www/
## access web page on your browser (http://127.0.0.1:8080)
```

## Use WASM FFI on node.js server side

A interesting thing about wasm, is that you can run it `not only` on webpages (frontends) but also on any platform that support wasm, as v8 (deno, nodejs)
wasmer, wasmtime, lucet, cloudflare workers.
And there is an interest on use it as a runtime target with containerization and isolation.

```bash
npx wasm-pack build --target nodejs --out-dir ./server/pkg
## It will compile for wasm on server/pkg/ folder and create binds for js
## go into server folder
## and run service
cd server/     # go into server folder
npm i          # install deps
npm run server # run service

## at another prompt curl POST
curl -X POST -H 'Content-Type: text/plain' localhost:3000 --data-binary "@samples/qgames.log"
```

---
## TODO
- Wasmer version
- Cloudflare workes version
- Build with docker
