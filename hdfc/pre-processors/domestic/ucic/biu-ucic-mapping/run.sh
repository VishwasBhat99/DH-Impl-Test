#!/usr/bin/env bash

INPUT_FILE=$"test-bed/biu.txt"
UCIC_MASTER_FILE=$"test-bed/ucic_master.csv"
CUSTOMER_BAL_FILE=$"test-bed/cust_bal.csv"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--ucic-master-file ${UCIC_MASTER_FILE} \
--customer-bal-file ${CUSTOMER_BAL_FILE} \
--input-field-delimiter "|" \
--ucic-field-delimiter "|" \
--customer-bal-file-delimiter "|" \
--is-cust-bal-footer false \
--as-on-date 27-01-2019 \
--log-level trace \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--diagnostics-flag true
