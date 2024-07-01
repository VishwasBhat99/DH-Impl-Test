#!/usr/bin/env bash

MASTER=$"test-bed/loan_lst.txt"
CASHFLOW=$"test-bed/acc_intrate_buffer_upt.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--master-input-file ${MASTER} \
--cashflow-input-file ${CASHFLOW} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
