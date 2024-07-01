#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-investments.txt"
OUTPUT=$"test-bed/cf-out-investments"
LOG_FILE=$"test-bed/cf-log-investments.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-investments.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-09-2019
