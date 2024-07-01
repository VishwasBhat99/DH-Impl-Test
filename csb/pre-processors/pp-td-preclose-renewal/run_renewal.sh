#!/usr/bin/env bash

INPUT=$"test-bed/$1"
OUTPUT=$"test-bed/$2"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EVENT="TDR"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--event-name ${EVENT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
