#!/usr/bin/env bash
set -euo pipefail

rm -rf build
mkdir -p build
mkdir -p build/iso

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

ld -m elf_x86_64 -T boot/linker.ld -o build/kernel.elf build/kernel.o
objcopy -O binary build/kernel.elf build/kernel.bin

size=$(wc -c < build/kernel.bin)
sector=$(((size + 511) / 512))
echo Kernel sectors $sector

echo "KERNEL_SECTORS equ $sector" > build/kernel_sectors.inc

nasm -f bin boot/bootloader.asm -o build/boot.bin
cat build/boot.bin build/kernel.bin > build/iso/crisnet-os.img

echo "Built: build/iso/crisnet-os.img"

qemu-system-x86_64 -drive format=raw,file=build/iso/crisnet-os.img
