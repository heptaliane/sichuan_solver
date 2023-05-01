# Sichuan Solver
Web solver for "Shisen-Sho" (四川省) which is the tile-based game with mahjong tiles.  
See [Wikipedia](https://en.wikipedia.org/wiki/Shisen-Sho) to know rules about "Shisen-Sho".

## Build
* Build with watch mode
``` bash
trunk serve
```
* Build for production
```
trunk serve --release
```


### How to setup build environment
1. Add rustup toolchain
``` bash
rustup toolchain install stable
```
2. Add wasm build target
``` bash
rustup target add wasm32-unknown-unknown
```
3. Install trunk
``` bash
cargo install trunk
```
