#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"
MASTER_FILE=$"test-bed/master.csv"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--master-sheet-name Sheet1 \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency INR \
--as-on-date 30-01-2026 \
--log-level trace \
--diagnostics-flag true
