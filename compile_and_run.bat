@echo off
cargo +nightly-x86_64-pc-windows-gnu build --target fledge.json

cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target\fledge\debug\bootimage-fledgeos-0.bin
