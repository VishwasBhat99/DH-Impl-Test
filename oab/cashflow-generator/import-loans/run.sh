#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-import-loans.txt"
OUTPUT=$"test-bed/cf-out-import-loans"
LOG_FILE=$"test-bed/cf-log-import-loans.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-import-loans.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 01-01-2009 \
--day-convention ACT/365
