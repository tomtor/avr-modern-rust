#!/bin/bash

CARGO_TARGET_DIR=target

#rustup component add rust-src # --toolchain $RUSTUP_TOOLCHAIN
cargo build --target-dir $CARGO_TARGET_DIR # --release

#if avr-objdump -d -C $CARGO_TARGET_DIR/avr-none/debug/avr_modern_demo.elf | fgrep 'mul	r16, r24'

SIZE=$(avr-size $CARGO_TARGET_DIR/avr-none/debug/avr_modern_demo.elf | tail -n 1 | cut -f 1)
echo $SIZE

if expr $SIZE \> 3756 > /dev/null ;
then
  echo Bad
  exit 1
else
  echo Ok
  exit 0
fi
