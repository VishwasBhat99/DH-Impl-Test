#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/undrwan-output"
LOG_FILE=$"test-bed/undrwan-log.txt"
DIAGNOSTICS_FILE=$"test-bed/undrwan-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 30-09-2020 \
--diagnostics-flag true 
