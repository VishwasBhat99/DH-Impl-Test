#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/td-output"
LOG_FILE=$"test-bed/td-log.txt"
DIAGNOSTICS_FILE=$"test-bed/td-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--as-on-date 21-01-2021 \
--diagnostics-flag true 
