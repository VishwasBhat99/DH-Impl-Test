#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
