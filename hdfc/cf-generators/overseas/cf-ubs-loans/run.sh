#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/cf-ubs-loans-output"
LOG_FILE=$"test-bed/cf-ubs-loans-log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-ubs-loans-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
