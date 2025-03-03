#!/bin/bash
set -e  # Exit on error
set -x  # Show commands being executed

# Set the manifest directory
export CARGO_MANIFEST_DIR=$(pwd)

cargo bootimage
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-gehri/debug/bootimage-gehri-os.bin \
    -display gtk \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -no-reboot \
    -no-shutdown \
    -d int,cpu_reset \
    -monitor stdio 