#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUT=$"test-bed/output.txt"
CON=$"test-bed/concat.txt"
NPA=$"test-bed/npa.txt"
SDL=$"test-bed/schedule.txt"
REC=$"test-bed/op-path.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/alm.xlsx"
CUST_MASTER=$"test-bed/cust-master.txt"
EXTRA_FIELDS=$"test-bed/extra-fields.txt"
LOAN_ADDITIONAL=$"test-bed/loan_add_short.txt"
LTV_FILE=$"test-bed/ltv.txt"
CARE_ACC_FILE=$"test-bed/care-acc.xlsx"
CARE_CUST_FILE=$"test-bed/care-cust.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--concat-file ${CON} \
--npa-file ${NPA} \
--schedule-file ${SDL} \
--rec-output-file ${REC} \
--cust-master ${CUST_MASTER} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--npa-sheet-name "NPA Consolidated Statements" \
--extra-fields-file-path ${EXTRA_FIELDS} \
--loan-additional-file-path ${LOAN_ADDITIONAL} \
--ltv-file ${LTV_FILE} \
--log-file ${LOG_FILE} \
--as-on-date 30-11-2019 \
--log-level trace \
--diagnostics-flag true \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
