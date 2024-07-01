#!/usr/bin/env bash

INPUT_FILE=$"test-bed/Excel_Utility.xlsx"
OUTPUT_PATH=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 23-05-2023 \
--row-num 2 \
--col-num 3 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--input-sheet-name Sheet2 \
--log-file ${LOG_FILE} \
--output-file ${OUTPUT_PATH}
#--log-level trace \
#--diagnostics-flag false
