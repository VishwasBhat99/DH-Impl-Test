#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
INT_RATE=$"test-bed/int_rate.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
PENALTY_FILE=$"test-bed/penalty_file.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--int-rate-file ${INT_RATE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--penalty-file {PENALTY_FILE} \
--as-on-date 27-01-2019
