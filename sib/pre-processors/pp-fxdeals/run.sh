#!/usr/bin/env bash


INPUT=$"test-bed/i1.txt"
EXC=$"test-bed/exc.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--output-file ${OUTPUT_FILE} \
--as-on-date  30-04-2023 \
--log-file ${LOG_FILE} \
--adj-fxdeals-file-path ${INPUT} \
--currency-prrate-file-path ${EXC} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
