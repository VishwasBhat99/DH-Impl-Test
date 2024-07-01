#!/usr/bin/env bash

BILLS_INPUT=$"test-bed/bills-input.txt"
OD_INPUT=$"test-bed/od-input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--bills-input-file ${BILLS_INPUT} \
--od-input-file ${OD_INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2023 \
--filter-bills-accs "Bills purchased and Discounted" \
--log-level trace \
#--diagnostics-flag true
