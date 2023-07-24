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

## Results :

- :x: : crash
- :heavy_check_mark: : no crash

| OS | Chrome/Chromium/Edge | Firefox | Webkit/Epiphany/Safari
| --- | :---: | :---: | :---: |
| Ubuntu / PopOs | :heavy_check_mark: | :heavy_check_mark: | :x:
| Fedora 37 | :heavy_check_mark: | :heavy_check_mark: | :x:
| Mac M1 | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: 


## Install Dependencies
```sh
cargo install trunk
```

