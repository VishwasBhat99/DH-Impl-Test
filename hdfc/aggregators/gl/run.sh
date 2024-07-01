#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/input.txt"
MERGE_FILE=$"test-bed/merge.txt"
MASTER_FILE=$"test-bed/master.xlsx"
EX=$"test-bed/1000ExchangeRate.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--merge-file ${MERGE_FILE} \
--master-file ${MASTER_FILE} \
--exchange-rate-file ${EX} \
--base-currency "INR" \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
