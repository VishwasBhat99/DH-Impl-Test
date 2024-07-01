#!/usr/bin/env bash

INPUT_FILE=$"test-bed/pp-out-overdraft.txt"
DLOD_CASHFLOW_FILE_1=$"test-bed/DLOD_cases.txt"
DLOD_CASHFLOW_FILE_2=$"test-bed/DLOD_cases_1.txt"
OUTPUT=$"test-bed/cf-out-dlod"
LOG_FILE=$"test-bed/cf-dlod-log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-dlod-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--dlod-file-1 ${DLOD_CASHFLOW_FILE_1} \
--dlod-file-2 ${DLOD_CASHFLOW_FILE_2} \
--output-file ${OUTPUT} \
--dlod-date-format-1 "%d-%m-%Y" \
--dlod-date-format-2 "%d-%m-%Y" \
--dlod-1-separator "," \
--dlod-2-separator "," \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-05-2023 
