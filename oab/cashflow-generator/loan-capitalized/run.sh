#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-loan-capitalized.txt"
OUTPUT=$"test-bed/cf-out-loan-capitalized"
LOG_FILE=$"test-bed/cf-log-loan-capitalized.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-loan-capitalized.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
