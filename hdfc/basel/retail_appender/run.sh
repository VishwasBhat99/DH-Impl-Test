#!/usr/bin/env bash

INPUT_FILE=$"test-bed/retail.txt"
REF_FILE=$"test-bed/EDW_ALM_CUSTID_TYPE.csv"
MASTER_FILE=$"test-bed/BIU.txt"
OUTPUT=$"test-bed/op/retail_output_file.txt"
LOG_FILE=$"test-bed/op/log.txt"
DIAGNOSTICS_FILE=$"test-bed/op/diag-log.txt"

cargo run  -- \
--input-file ${INPUT_FILE} \
--ref-file ${REF_FILE} \
--master-file ${MASTER_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level trace \
--diagnostics-flag true
