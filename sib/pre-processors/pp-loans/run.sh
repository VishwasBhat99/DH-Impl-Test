#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
LOANS_INPUT_FILE=$"test-bed/loans-input.txt"
OVERDUE_INPUT_FILE=$"test-bed/loans-overdue.txt"
ACH_INPUT_FILE=$"test-bed/ach-input.txt"
EIT_INPUT_FILE=$"test-bed/eit-input.txt"
EAB_INPUT_FILE=$"test-bed/eab-input.txt"
LAM_INPUT_FILE=$"test-bed/lam-input.txt"
INTRATE_INPUT_FILE=$"test-bed/loans-intrate.txt"
RATE_CODE=$"test-bed/Rate_Code.txt"
LOAN_ADD_FILE=$"test-bed/LOAN_ADD_FILE.txt"
OUTPUT_FILE=$"test-bed/output.txt"
NPA=$"test-bed/npa.txt"

cargo run --release -- \
--input-file ${LOANS_INPUT_FILE} \
--overdue-input-file ${OVERDUE_INPUT_FILE} \
--ach-input-file ${ACH_INPUT_FILE} \
--itc-input-file ${EAB_INPUT_FILE} \
--eit-input-file ${EIT_INPUT_FILE} \
--npa-input-file ${NPA} \
--lam-input-file ${EIT_INPUT_FILE} \
--intrate-input-file ${INTRATE_INPUT_FILE} \
--rate-code-mapping-master ${RATE_CODE} \
--loan-add-file ${LOAN_ADD_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  30-06-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
