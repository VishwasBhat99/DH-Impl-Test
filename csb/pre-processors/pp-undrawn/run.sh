#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/CFOutput"
CUST_MASTER=$"test-bed/cust_master.txt"
OUTPUT=$"test-bed/output.txt"
EXTRA_FIELDS=$"test-bed/extra_fields.txt"
LTV_FILE=$"test-bed/ltv_file.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--cust-master ${CUST_MASTER} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--extra-fields-file-path ${EXTRA_FIELDS} \
--ltv-file ${LTV_FILE} \
--as-on-date 28-02-2018 
