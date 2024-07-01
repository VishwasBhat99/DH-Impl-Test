#!/usr/bin/env bash

INPUT=$"test-bed/Input.txt"
INT_RATE=$"test-bed/int_rate.txt"
RATECODE=$"test-bed/ratecode.txt"
NPA=$"test-bed/npa.txt"
CONFIG=$"test-bed/config.txt"
TBL_CODE=$"test-bed/tbl_code.txt"
ADDITIONAL_FILE=$"test-bed/Additional_Loan_File.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--int-rate-file ${INT_RATE} \
--config-file-path ${CONFIG} \
--npa-file-path ${NPA} \
--ratecode-file-path ${RATECODE} \
--tbl-code-file ${TBL_CODE} \
--additional-loan-file ${ADDITIONAL_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
