# RUST - notes

### Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#~/.cargo/bin - It has rustc, cargo, and rustup binaries
# restart the terminal
rustc --version # check version
rustup update # update rust
rustup self uninstall # uninstall rust
```

### Commands

```bash
cargo new rust-app # it creates new app called rust-app
cd rust-app
cargo run # compile and run the app
cargo build # Build artifacts inside target folder
./rust-app
cargo build --release #Build artifacts in release mode, with optimizations
```
