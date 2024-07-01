#!/usr/bin/env bash

INPUT=$"test-bed/AdjRates.txt"
OUTPUT=$"test-bed/adj-out.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--derive-till-n-days 5 \
--log-level trace \
--diagnostics-flag true \
