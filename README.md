# Learn ARM

```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
rustup component add llvm-tools
cargo install cargo-binutils
sudo apt install gdb-multiarch openocd qemu-system-arm libudev-dev
```

Install `probe-rs`: https://probe.rs

Run binaries: `cargo run --release --bin blinky`
