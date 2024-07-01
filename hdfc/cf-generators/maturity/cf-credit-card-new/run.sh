#!/usr/bin/env bash

INPUT=$"test-bed/Input.txt"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"


cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--is-header-present true \
--as-on-date "01-01-2019" \
--log-level trace \
--diagnostics-flag true
