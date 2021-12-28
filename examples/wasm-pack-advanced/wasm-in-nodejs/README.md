## Run this example
From the blog [trustyrust](https://trustyrust.com/blog/wasm-pack-advanced)

### What is this Example?
This is an example demonstrates how to run Rust code in NodeJs via building to wasm.

### first build the wasm from the rust code
```sh
cd wasm-from-rust
wasm-pack build --out-dir wasm-nodejs --target nodejs
```

### run this nodejs project
```sh
npm i
tsc src/index.ts --outDir lib && node lib/index.js
```
