# tablify
Turn a tabular data into a table element.

- web interface: https://ykszk.github.io/tablify/
- command line interface: https://github.com/ykszk/tablify/releases

# Development

## ARCHITECTURE
- Rust: Core functionality -> wasm + CLI
- React: Web app


## Setup
Install rust and nodejs.

## CLI
```sh
cargo test
```

## Web interface
```sh
cd www
cargo install wasm-pack
npm run build-wasm
npm install
npm run dev
```
