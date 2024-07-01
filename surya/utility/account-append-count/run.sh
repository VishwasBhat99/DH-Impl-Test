#!/usr/bin/env bash

INPUT_FILE=$"test-bed/pp-out-inv.txt"
OUTPUT_FILE=$"test-bed/pp-suffix-output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file-path ${INPUT_FILE} \
--output-file-path ${OUTPUT_FILE} \
--as-on-date 30-06-2022 \
--delimiter '|' \
--log-level trace \
--diagnostics-flag false
