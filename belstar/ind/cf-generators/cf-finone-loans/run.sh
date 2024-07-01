#!/usr/bin/env bash

INPUT=$"test-bed/finnone-loans-ppoutput.txt"
OUTPUT=$"test-bed/cf-finnone-loans-output"
LOG_FILE=$"test-bed/cf-finnone-loans-log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-finnone-loans-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention-type ACTby365 \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-08-2022 
