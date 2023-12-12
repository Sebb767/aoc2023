#!/bin/bash
if [ -z "$1" ]; then
	echo "Usage: $0 <day>"
	echo "i.e.: $0 8"
	exit 0
fi

[ -f "day$1.rs" ] && echo "File day$1.rs exists!" && exit 1

sed <dayn.rs -e "s/dayn/day$1/g" -e "s/n-1/$1-1/g" > day$1.rs
