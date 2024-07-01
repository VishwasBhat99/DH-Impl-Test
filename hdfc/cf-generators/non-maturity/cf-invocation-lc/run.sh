#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/cf-out-invocation-lc"
LOG_FILE=$"test-bed/cf-log-invocation-lc.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-invocation-lc.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
