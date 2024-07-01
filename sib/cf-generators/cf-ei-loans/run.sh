#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REPAY=$"test-bed/res.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OVERDUE=$"test-bed/ovd.txt"

cargo run --release -- \
--input-file ${INPUT} \
--repayment-struct-file ${REPAY} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--adj-cf-type LAST \
--log-level trace \
--overdue-input-file ${OVERDUE} \
--as-on-date 30-06-2023 \
--diagnostics-flag true 
