#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-rd.txt"
OUTPUT=$"test-bed/cf-out-rd"
LOG_FILE=$"test-bed/cf-rd-log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-rd-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-08-2022 
