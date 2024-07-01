#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
OUTPUT=$"test-bed/TDCFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date $2 \
--day-convention ACT/365
#--log-level trace \
#-diagnostics-flag true
