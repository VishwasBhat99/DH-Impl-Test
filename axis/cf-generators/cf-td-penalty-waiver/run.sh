#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/cfoutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 31-03-2023 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INP_FILE} \
--log-file ${LOG_FILE} \
--output-file ${OUTPUT_FILE} 
