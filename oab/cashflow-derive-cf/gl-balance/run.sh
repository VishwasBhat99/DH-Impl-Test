#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-gl-balance.txt"
OUTPUT=$"test-bed/cf-out-gl-balance"
LOG_FILE=$"test-bed/cf-log-gl-balance.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-gl-balance.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
