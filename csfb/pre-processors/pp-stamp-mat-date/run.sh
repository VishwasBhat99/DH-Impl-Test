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
--as-on-date 31-03-2023 \
--log-level trace \
--diagnostics-flag true
