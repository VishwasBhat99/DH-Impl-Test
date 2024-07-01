#!/usr/bin/env bash

INPUT=$"testbed/hmm3025.xls"
OUT=$"testbed/hmm3025.txt"
LOG_FILE=$"testbed/log.txt"
DIAGNOSTICS_FILE=$"testbed/diag-log.txt"
SHEET_NAME=$"Sheet1"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
--sheet-name ${SHEET_NAME} \
