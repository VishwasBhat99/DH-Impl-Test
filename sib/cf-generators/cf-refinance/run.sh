#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file-path ${INPUT_FILE} \
--output-file-path ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-06-2023
