#!/usr/bin/env bash

DAY_DIS_FILE=$"test-bed/day-dis.txt"
OP_FILE=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--as-on-date 31-03-2021 \
--existing-business-value 100000 \
--prj-business-value 120000 \
--currency INR \
--disbursement-by-day-file-path ${DAY_DIS_FILE} \
--output-file-path ${OP_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
--diagnostics-flag false
