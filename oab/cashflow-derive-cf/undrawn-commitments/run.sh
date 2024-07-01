#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-undrawn-commitments.txt"
OUTPUT=$"test-bed/cf-out-undrawn-commitments"
LOG_FILE=$"test-bed/cf-log-undrawn-commitments.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-undrawn-commitments.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
