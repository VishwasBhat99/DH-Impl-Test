#!/usr/bin/env bash

TopNCustAcc=$"test-bed/dep_acc.txt"
TopNIntRate=$"test-bed/int_rate.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--top-n-cust-acc-file ${TopNCustAcc} \
--top-n-int-rate-file ${TopNIntRate} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--base-currency "INR" \
--country-id "GC" \
--diagnostics-flag true \
--as-on-date 31-05-2023
