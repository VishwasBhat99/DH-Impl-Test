#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency INR \
--as-on-date 28-02-2023 \
--log-level trace \
--diagnostics-flag true \
--input-date-format "dd-mm-yy"
