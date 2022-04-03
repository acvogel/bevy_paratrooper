### Paratrooper

## Webasm Build
- Disable kira audio plugin dependency and plugin
- `cargo build --release --target wasm32-unknown-unknown`
- `wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/bevy_paratrooper.wasm`
- Update `./out/` files on `web` branch

## Assets
https://opengameart.org/content/war-on-water-gfx

