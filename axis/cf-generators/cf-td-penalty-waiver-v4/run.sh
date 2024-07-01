#!/usr/bin/env bash

INP_FILE=$"test-bed/test.txt"
OUTPUT_FILE=$"test-bed/cfoutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 05-02-2024 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INP_FILE} \
--log-file ${LOG_FILE} \
--skip-header false \
--output-file ${OUTPUT_FILE} \
--source-name "001" 
