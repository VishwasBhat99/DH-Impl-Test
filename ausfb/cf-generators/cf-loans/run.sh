#!/usr/bin/env bash

INPUT=$"test-bed/input-file.csv"
INPUT_CASHFLOW_FILE=$"test-bed/cf-input-file.txt"
OUTPUT=$"test-bed/cf-out-loans"
LOG_FILE=$"test-bed/cf-cf-loans--log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-cf-loans-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--input-cf-file ${INPUT_CASHFLOW_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-08-2022 
