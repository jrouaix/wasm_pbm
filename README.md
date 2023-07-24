# wasm_pbm

### Following the issue described by @t0m in here :
- https://github.com/tauri-apps/tauri/issues/7154
- https://github.com/rustwasm/wasm-bindgen/issues/3470
- https://github.com/leptos-rs/leptos/issues/1153

### Trying to have a minimalist reproducible example of the issue.

![if you see this](crash.png)

## Run examples :

```sh
cd leptos_example
trunk serve --open
```

open `http://locahost:8080` on you browser and wait for it to crash, then open the console to see the error.


## Install Dependencies
```sh
cargo install trunk
```