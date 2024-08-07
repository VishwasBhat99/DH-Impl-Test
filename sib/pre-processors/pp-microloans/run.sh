#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
CF_FILE=$"test-bed/cashflows.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
NPA=$"test-bed/npa.txt"

cargo run -- \
--input-file-path ${INPUT} \
--cashflow-file-path ${CF_FILE} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--npa-file-path ${NPA} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-06-2023
