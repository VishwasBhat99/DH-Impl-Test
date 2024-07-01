#!/usr/bin/env bash

INPUT=$"test-bed/blr.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file-path ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 18-06-2020 \
--log-level trace \
--diagnostics-flag true