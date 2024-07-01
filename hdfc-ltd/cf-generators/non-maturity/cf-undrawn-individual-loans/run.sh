#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
PER_FILE=$"test-bed/per_file.xlsx"
OUTPUT=$"test-bed/undrwan-output"
LOG_FILE=$"test-bed/undrwan-log.txt"
DIAGNOSTICS_FILE=$"test-bed/undrwan-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--percentage-file ${PER_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 01-02-2022 \
--diagnostics-flag true 
