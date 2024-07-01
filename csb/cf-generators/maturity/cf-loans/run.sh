#!/usr/bin/env bash

INPUT=$"test-bed/24112020/loans30112020.txt"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag.txt"
batch_size=500000
num_threads=6


cargo run -- \
--batch-size ${batch_size} \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--day-convention ACT/365 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--num-threads ${num_threads} \
--as-on-date 30-11-2020 \
--is-contractual true \
--log-level trace \
--diagnostics-flag true
