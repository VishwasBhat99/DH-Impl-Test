#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER=$"test-bed/master.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--master-file ${MASTER} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 12-07-2022 \
--log-level trace \
--diagnostics-flag true
