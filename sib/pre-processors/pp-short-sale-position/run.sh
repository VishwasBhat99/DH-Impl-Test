#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/op.txt"
MASTER=$"test-bed/mr_timeband.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--mr-timeband ${MASTER} \
--output-file ${OUTPUT} \
--as-on-date 30-08-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level info \
--diagnostics-flag true \