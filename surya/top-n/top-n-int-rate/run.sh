#!/usr/bin/env bash

TopNCustAcc=$"test-bed/input1.txt"
TopNCustProd=$"test-bed/input2.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--top-n-cust-acc-file ${TopNCustAcc} \
--top-n-cust-prod-file ${TopNCustProd} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-05-2023
