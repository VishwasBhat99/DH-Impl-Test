#!/usr/bin/env bash

INPUT_FILE=$"test-bed/DCF_TCL_FY24.xlsx"
OUTPUT_FILE=$"test-bed/out"
LOG_FILE=$"test-bed/log.txt"
DIAG_LOG_FILE=$"test-bed/diag-log.txt"


cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--skip-rows "1,2,3,4,11,19,20,27,32,35,36,39-43,GT50" \
--currency "INR" \
--denomination "10" \
--as-on-date 15-01-2024 \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAG_LOG_FILE} \
--log-level "info" \
--diagnostics-flag "false" 
