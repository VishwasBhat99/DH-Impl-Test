#!/usr/bin/env bash

INPUT=$"test-bed/output.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTbyACT \
--log-level trace \
--diagnostics-flag false \
--as-on-date 31-01-2019
