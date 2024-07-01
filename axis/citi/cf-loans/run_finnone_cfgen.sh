#!/usr/bin/env bash

INPUT=$"test-bed/output.txt"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag.txt"
batch_size=500000
num_threads=6


cargo run --release -- \
--batch-size ${batch_size} \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--day-convention 30/360 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--num-threads ${num_threads} \
--as-on-date 21-02-2019
#--log-level trace \
#--diagnostics-flag true