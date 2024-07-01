#!/usr/bin/env bash

INPUT=$"test-bed/output.txt"
OUTPUT=$"test-bed/cf_purchase_portfolio"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 30-09-2023 \
--day-convention ACT/365 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT} \
--log-file ${LOG_FILE} \
--output-file ${OUTPUT}