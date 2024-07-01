#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/mf-output"
LOG_FILE=$"test-bed/mf-log.txt"
DIAGNOSTICS_FILE=$"test-bed/mf-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 30-09-2020 \
--diagnostics-flag true 
