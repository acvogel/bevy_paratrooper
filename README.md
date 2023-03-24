### Paratrooper

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