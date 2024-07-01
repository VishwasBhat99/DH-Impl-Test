#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-loannonei.txt"
REPAY=$"test-bed/res_non_emi.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
HOLIDAY=$"test-bed/holiday.txt"

cargo run --release -- \
--input-file ${INPUT} \
--repayment-struct-file ${REPAY} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--holiday-yearccy-file ${HOLIDAY} \
--as-on-date 30-09-2022 \
--diagnostics-flag true 
