#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/fd-output"
LOG_FILE=$"test-bed/fd-log.txt"
DIAGNOSTICS_FILE=$"test-bed/fd-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 31-01-2021 \
--diagnostics-flag true 
