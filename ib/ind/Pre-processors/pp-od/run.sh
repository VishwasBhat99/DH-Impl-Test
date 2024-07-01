#!/usr/bin/env bash

INPUT_FILE=$"test-bed/OD.txt"
NPA_INPUT_FILE=$"test-bed/npa.txt"
OUTPUT_FILE=$"test-bed/output.txt"
MASTER_FILE=$"test-bed/master.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--npa-input-file ${NPA_INPUT_FILE} \
--master-file ${MASTER_FILE} \
--output-file ${OUTPUT_FILE} \
--date-fields "12|13|14|18|39" \
--master-sheet-name "LLG Master" \
--next-rep-file ${MASTER_FILE} \
--header-rows "" \
--as-on-date 30-04-2023 \
--log-level trace \
--diagnostics-flag true
