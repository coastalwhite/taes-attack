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

link ../pulpino-top-level-cw305/program/ext/connection.py python-srcs/connection.py
link ../python-srcs cw305/taes
link ../python-srcs gem5/taes
link ../pulpino-top-level-cw305/program/target/to_ram.py cw305/to_ram.py
link ../pulpino-top-level-cw305/program/target/Linkerscript-RAM.lds att/Linkerscript-RAM.lds
