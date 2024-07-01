#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
EDW_MASTER_FILE=$"test-bed/edw_master.txt"
UCIC_BIU_FILE=$"test-bed/ucic_biu.txt"
CUSTOMER_BAL_FILE=$"test-bed/cust_bal.txt"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--ucic-biu-file ${UCIC_BIU_FILE} \
--customer-bal-file ${CUSTOMER_BAL_FILE} \
--edw-master-file ${EDW_MASTER_FILE} \
--input-file-delimiter "|" \
--ucic-file-delimiter "|" \
--customer-bal-file-delimiter "|" \
--edw-file-delimiter "~#~" \
--as-on-date 27-01-2019 \
--log-level trace \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--diagnostics-flag true
