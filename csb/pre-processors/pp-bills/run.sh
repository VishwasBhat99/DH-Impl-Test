#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REF1=$"test-bed/alm.xlsx"
REF2=$"test-bed/npa.txt"
OUTPUT=$"test-bed/output.txt"
CONCAT=$"test-bed/concat.txt"
BLDYN=$"test-bed/bills-short.txt"
RECOUT=$"test-bed/op-path.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CUST_MASTER=$"test-bed/cust-master.txt"
BANK_MASTER=$"test-bed/bank-master.xlsx"
EXTRA_FIELDS=$"test-bed/extra-fields.txt"
LOAN_ADDITIONAL=$"test-bed/loan_add_short.txt"
LTV_FILE=$"test-bed/ltv.txt"
CARE_ACC_FILE=$"test-bed/care-acc.xlsx"
CARE_CUST_FILE=$"test-bed/care-cust.xlsx"

cargo run --release -- \
--input-file-path ${INPUT} \
--alm-master ${REF1} \
--npa-consolidated ${REF2} \
--alm-master-sheet-name "Sheet1" \
--cust-master ${CUST_MASTER} \
--npa-sheet-name "Sheet1" \
--output-file-path ${OUTPUT} \
--rec-output-file-path ${RECOUT} \
--concat-file-path ${CONCAT} \
--bills-dyn ${BLDYN} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--bank-master ${BANK_MASTER} \
--bank-master-sheet-name "Sheet1" \
--extra-fields-file-path ${EXTRA_FIELDS} \
--loan-additional-file-path ${LOAN_ADDITIONAL} \
--ltv-file ${LTV_FILE} \
--as-on-date 08-09-2021 \
--log-level trace \
--diagnostics-flag true
