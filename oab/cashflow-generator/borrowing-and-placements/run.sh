#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-placements.txt"
OUTPUT=$"test-bed/cf-out-placements"
LOG_FILE=$"test-bed/cf-out-log-placements.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-out-diag-log-placements.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
