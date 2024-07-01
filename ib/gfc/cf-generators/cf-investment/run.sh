#!/usr/bin/env bash

INPUT_FILE=$"test-bed/invest.txt"
OUTPUT=$"test-bed/TDCFOutput"
LOG_FILE=$"test-bed/log/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log/diag-log.txt"




cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2023 \
#--log-level trace \
#-diagnostics-flag true
