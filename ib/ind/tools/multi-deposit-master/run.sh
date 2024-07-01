#!/usr/bin/env bash

CA_INPUT=$"test-bed/ca.txt"
TD_INPUT_FILE=$"test-bed/td.txt"
SA_INPUT_FILE=$"test-bed/casa.txt"
OUTPUT_FILE=$"test-bed"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--td-input-file ${TD_INPUT_FILE} \
--sa-input-file ${SA_INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--ca-input-file ${CA_INPUT} \
--is-cust-repeated true \
--as-on-date 31-01-2022 
#--log-level trace \
#--diagnostics-flag true
