#!/usr/bin/env bash
set -euo pipefail

rm -rf build
mkdir -p build

rustc \
  --target x86_64-unknown-none \
  -C relocation-model=static \
  -C code-model=small \
  -C panic=abort \
  -C opt-level=2 \
  -C overflow-checks=no \
  --emit=obj \
  -o build/kernel.o \
  src/main.rs

ld -m elf_x86_64 -T src/linker.ld -o build/kernel.elf build/kernel.o
objcopy -O binary build/kernel.elf build/kernel.bin

#size=$(wc -c < build/kernel.bin)
#if [ "$size" -gt 1024 ]; then
#    echo "Kernel is too large: ${size} bytes (maximum 1024 bytes)."
#    exit 1
#fi

#truncate -s 1024 build/kernel.bin
nasm -f bin src/bootloader.asm -o build/boot.bin
cat build/boot.bin build/kernel.bin > build/crisnet-os.img

echo "Built: build/crisnet-os.img"

qemu-system-x86_64 -drive format=raw,file=build/crisnet-os.img
