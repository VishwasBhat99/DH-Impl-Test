#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--file-format "txt" \
--input-sheet-name "Sheet1" \
--input-delimeter  "|" \
--amt-pos 2 \
--head-count 2 \
--foot-count 1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#-diagnostics-flag true
