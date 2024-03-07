# Paratrooper

## Play from source code (Linux and MacOS)
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone repository: `git clone git@github.com:acvogel/bevy_paratrooper.git`
3. Compile and run: `cd bevy_paratrooper; cargo run --release`

## Webasm Build
- Disable kira audio plugin dependency and plugin
- `cargo build --release --target wasm32-unknown-unknown`
- `wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/bevy_paratrooper.wasm`
- Update `./out/` files on `web` branch

## Windows Build
`cargo build --target=x86_64-pc-windows-gnu --release`

## Assets
- https://opengameart.org/content/war-on-water-gfx
- https://opengameart.org/content/512-sound-effects-8-bit-style

## itch.io publish
butler push paratrooper.zip mbusux/paratrooper:linux --userversion 0.1
