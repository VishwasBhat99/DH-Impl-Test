#!/usr/bin/env bash

INPUT=$"test-bed/01062021/input_01062021.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--req-cols "1|7|2|9" \
--as-on-date "01-06-2021" \
--date-format "DDMMYYYY" \
--task "delete" \
--diagnostics-flag true 
