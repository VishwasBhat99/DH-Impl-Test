#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"
MASTER_FILE=$"test-bed/master.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--output-file ${OUTPUT_FILE} \
--sheet-name "LLG Master" \
--as-on-date 31-01-2022 \
--input-date-format dd-mmm-yy
#--log-level trace \
#--diagnostics-flag true
