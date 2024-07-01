#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--precision 6 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
