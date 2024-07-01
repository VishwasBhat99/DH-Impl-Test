#!/usr/bin/env bash

INPUT=$"test-bed/test"
OUTPUT=$"test-bed/bills-output"
LOG_FILE=$"test-bed/bills-log.txt"
DIAGNOSTICS_FILE=$"test-bed/bills-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 31-01-19 \
--diagnostics-flag true 
