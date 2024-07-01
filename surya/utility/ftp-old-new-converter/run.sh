#!/usr/bin/env bash

INPUT=$"test-bed/new_input.txt"
OUTPUT=$"test-bed/Bills.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--output-file-path ${OUTPUT} \
--as-on-date 27-01-2019
