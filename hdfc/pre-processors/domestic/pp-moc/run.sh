#!/usr/bin/env bash

INPUT_FILE=$"test-bed/moc.xlsx"
SHEET_NAME=$"Sheet1"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--sheet-name ${SHEET_NAME} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2020 \
--log-level trace \
--diagnostics-flag true
