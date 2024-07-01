#!/usr/bin/env bash

INPUT=$"test-bed/rates.txt"
OUTPUT=$"test-bed/output/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

rm test-bed/output/*

cargo run --release --  \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 07-01-2020
# --log-level trace \
# --diagnostics-flag true 
