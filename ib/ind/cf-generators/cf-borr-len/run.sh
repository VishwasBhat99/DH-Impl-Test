#!/usr/bin/env bash

INPUT=$"test-bed/borr-len-ppoutput.txt"
OUTPUT=$"test-bed/cf-borr-len-output"
LOG_FILE=$"test-bed/cf-borr-len--log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-borr-len-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-08-2022 
