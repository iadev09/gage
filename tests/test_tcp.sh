#!/usr/bin/env bash

python3 -c 'import sys,struct;sys.stdout.buffer.write(struct.pack(">I", len(sys.argv[1].encode())) + sys.argv[1].encode())' "$1" \
	| nc 127.0.0.1 6000
