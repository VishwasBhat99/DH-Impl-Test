#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CUST_FILE=$"test-bed/cust_master.txt"

cargo run -- \
--ucic-master-file ${INPUT} \
--cust-master-file ${CUST_FILE} \
--log-file ${LOG_FILE} \
--output-file ${OUTPUT} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ucic-field-delimiter "|" \
--cust-field-delimiter "|" \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true