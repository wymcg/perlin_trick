# Perlin Noise Trick
A [Matricks](https://github.com/wymcg/matricks) plugin that makes some pretty colors using [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise).

## Build
- Install the `wasm32-wasi` toolchain by running `rustup target add wasm32-wasi`
- Run `cargo build --release --target wasm32-wasi`
- Run the plugin with [Matricks](https://github.com/wymcg/matricks) (on a Raspberry Pi) or with [Simtricks](https://github.com/wymcg/simtricks) (on other devices).
