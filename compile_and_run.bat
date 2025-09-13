@echo off
cargo +nightly-x86_64-pc-windows-gnu build --target fledge.json

rust-objcopy -O binary target\fledge\debug\fledgeos-0 kernel.bin

qemu-system-x86_64 -kernel kernel.bin