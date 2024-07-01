#!/usr/bin/env bash

file=$"test-bed/sample"

cargo run --release --  \
-o $file \
-a  \
-f 2 \
-s 998 \
-n 2000 \
-u 2000 \
-c 98
