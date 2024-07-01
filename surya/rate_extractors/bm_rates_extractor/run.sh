#!/usr/bin/env bash

INPUT=$"test-bed/rates.txt"
OUTPUT=$"test-bed/output/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

rm test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-05-2022 \
--no-avg-days 5 \
--skip-bmid-vec 1002,1003 \
--skip-date-vec 08-15,01-26,10-02,11-14,09-05 \
# --log-level trace \
# --diagnostics-flag true
