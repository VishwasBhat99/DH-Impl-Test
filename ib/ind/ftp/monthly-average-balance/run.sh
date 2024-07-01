#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/monthavgbal.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER=$"test-bed/master.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2023 \
--default-method-id "1001" \
--master-file ${MASTER} \
--master-sheet-name "LLG Master" \
--reference-date 31-03 \
#--log-level trace \
#--diagnostics-flag true
