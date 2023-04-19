#!/bin/sh

ATT_PATH="../att"

if [ -z $1 ] ; then
    echo "Usage: $0 <path/to/attack/dir>"
    exit 2
fi

ATT_DIR="$1"
ATT_BIN="$ATT_PATH/$ATT_DIR/target/riscv32i-unknown-none-elf/release/$ATT_DIR"

set -e

pushd "$ATT_PATH/$ATT_DIR" > /dev/null
cargo build --release --no-default-features --features cw305
popd > /dev/null

riscv-none-elf-objdump -D "$ATT_BIN" > dump.out
./to_ram.py dump.out program.py
./interaction.py "$ATT_DIR"