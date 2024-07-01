#!/usr/bin/env bash

INPUT=$"test-bed/EDW_ALM_TD_RATES_20200930.csv"
OUTPUT=$"test-bed/01-03-2020.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true

