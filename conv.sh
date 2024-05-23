#!/bin/bash

for file in 2*.csv; do tmpfile="/tmp/${file}.tmp"; iconv -f sjis -t utf8 "$file" > "$tmpfile" && mv "$tmpfile" "$file"; done