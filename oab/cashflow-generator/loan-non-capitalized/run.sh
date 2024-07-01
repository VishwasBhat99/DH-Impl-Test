#!/usr/bin/env bash

INPUT=$"test-bed/sample.txt"
OUTPUT=$"test-bed/cf-out-loan-non-capitalized"
LOG_FILE=$"test-bed/cf-log-loan-non-capitalized.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-loan-non-capitalized.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--precision 3 \
--diagnostics-flag true \
--as-on-date 30-09-2019
