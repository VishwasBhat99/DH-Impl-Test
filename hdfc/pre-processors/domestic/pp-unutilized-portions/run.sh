#!/usr/bin/env bash

INPUT1=$"test-bed/UBS_Export.xlsx"
INPUT2=$"test-bed/UBS_Export.xlsx"
INPUT3=$"test-bed/UBS_Export.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
SHEET_NAME=$"Sheet1"

cargo run --release -- \
--input-file-1 ${INPUT1} \
--input-file-2 ${INPUT2} \
--input-file-3 ${INPUT3} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true\
--sheet-name ${SHEET_NAME} \
