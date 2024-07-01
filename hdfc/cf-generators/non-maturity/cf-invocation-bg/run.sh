#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-invocation-bg.txt"
OUTPUT=$"test-bed/cf-out-invocation-bg"
LOG_FILE=$"test-bed/cf-log-invocation-bg.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-invocation-bg.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
