#!/bin/sh

# Load GEM5_DIR
source ../settings.sh

ATT_PATH="../att"

if [ -z $1 ] ; then
    echo "Usage: $0 <path/to/attack/dir>"
    exit 2
fi

ATT_DIR="$1"
ATT_BIN="$ATT_PATH/$ATT_DIR/target/riscv32i-unknown-none-elf/release/$ATT_DIR"

set -e


pushd "$ATT_PATH/$ATT_DIR" > /dev/null
cargo build --release
popd > /dev/null

if [ -f "output.txt" ] ; then
	rm output.txt
fi

printf "" > input.txt

("$GEM5_DIR/build/RISCV/gem5.opt" ./system.py "$ATT_BIN" > output.txt) & GEMPID=$!
./interaction.py "$ATT_DIR"
kill $GEMPID