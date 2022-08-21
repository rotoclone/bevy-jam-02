# Mister Smartyplants

For Bevy Jam 2: https://itch.io/jam/bevy-jam-2

Play it on itch.io: TODO link here

Licensed under the dual MIT / Apache-2.0 license

## Building for web
### Prerequisites
* `rustup target install wasm32-unknown-unknown`
* `cargo install wasm-bindgen-cli`
### Build
1. `cargo build --release --target wasm32-unknown-unknown`
1. `wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/bevy-jam-02.wasm`
1. `cp index.html out`
1. `cp -r assets out`
