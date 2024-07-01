#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  20-01-2021 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
