#!/usr/bin/env bash

INPUT=$"test-bed/singleeicfinput.txt"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-06-2021 \
# --log-level trace \
# --diagnostics-flag true \
