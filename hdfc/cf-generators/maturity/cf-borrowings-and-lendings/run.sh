#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
SUB=$"test-bed/sub_dept.xlsx"
OUTPUT=$"test-bed/cf-len-output"
OUTPUT=$"test-bed/a1"
LOG_FILE=$"test-bed/bor-log.txt"
DIAGNOSTICS_FILE=$"test-bed/bor-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--sub-dept-file ${SUB} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--sub-dept-sheet-name Sheet1 \
--as-on-date 28-02-2019 \
--log-level trace \
--diagnostics-flag true 
 