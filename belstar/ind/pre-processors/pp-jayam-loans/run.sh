#!/usr/bin/env bash

INPUT_MASTER=$"test-bed/Jayam_Master.txt"
CASHFLOW_INPUT=$"test-bed/Jayam_CF.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-master-file ${INPUT_MASTER} \
--input-cashflow-file ${CASHFLOW_INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-06-2022 \
--log-level trace \
--diagnostics-flag true
