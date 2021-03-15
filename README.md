# tablify
Turn a tabular data into a table element.

## ARCHITECTURE
- Rust: Core functionality -> wasm + CLI
- React: Web app

## Development

### Setup
```sh
cargo install wasm-pack
```

### Rust
```sh
cargo build
```

### React
```sh
wasm-pack build
cd www
yarn build
```
