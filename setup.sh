#! /bin/sh

git submodule update --init

function link() {
	FROM="$1"
	TO="$2"

	echo "[LINK]: $1 -> $2"

	if [ ! -e "$TO" ] ; then
		ln -s "$FROM" "$TO"
	fi
}

if [ ! -z "$GEM5_DIR" ]; then
    echo "[SETTINGS]: Settings GEM5_DIR to '$GEM5_DIR'"
    echo "GEM5_DIR=$GEM5_DIR" > ./settings.sh
else
    echo "[ERROR]: GEM5_DIR is not set"
    exit 1
fi

link ../pulpino-top-level-cw305/program/ext/connection.py python-srcs/connection.py
link ../python-srcs cw305/taes
link ../python-srcs gem5/taes
link ../pulpino-top-level-cw305/program/target/to_ram.py cw305/to_ram.py
link ../pulpino-top-level-cw305/program/target/Linkerscript-RAM.lds att/Linkerscript-RAM.lds
