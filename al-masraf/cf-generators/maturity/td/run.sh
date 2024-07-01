#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-td.txt"
OUTPUT=$"test-bed/cf-out-td"
LOG_FILE=$"test-bed/cf-log-td.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-td.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 \
--day-convention ACT/ACT
