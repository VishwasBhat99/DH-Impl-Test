#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/bills-output"
LOG_FILE=$"test-bed/bills-log.txt"
DIAGNOSTICS_FILE=$"test-bed/bills-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--as-on-date 22-10-22 \
--diagnostics-flag true 
