#!/usr/bin/env bash

PSL_CATEG_FILE=$"test-bed/psl_category.txt"
DAILY_BAL=$"test-bed/daily_bal.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--psl-category-file ${PSL_CATEG_FILE} \
--daily-bal-file ${DAILY_BAL} \
--incentive-rate 1 \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2023 \
#--log-level trace \
#--diagnostics-flag true
