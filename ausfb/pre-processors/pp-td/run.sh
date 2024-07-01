#!/usr/bin/env bash

INPUT=$"test-bed/td_inpp.csv"
OUTPUT=$"test-bed/output.txt"
REC=$"test-bed/rec_output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--rec-output-file ${REC} \
--source-name "TD" \
--as-on-date 27-01-2019
