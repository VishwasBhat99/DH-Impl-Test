#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-lc-acceptances.txt"
OUTPUT=$"test-bed/cf-out-lc-acceptances"
LOG_FILE=$"test-bed/cf-log-lc-acceptances.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-lc-acceptances.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
