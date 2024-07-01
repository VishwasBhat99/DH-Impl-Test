#!/usr/bin/env bash

INPUT=$"test-bed/Add_file_HKLMR_Sample.xlsx"
OUT=$"test-bed/output.cf"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--input-sheet-name Sheet1 \
--output-file ${OUT} \
--denomination 100000 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
