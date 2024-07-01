#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
LOAN_ACCT_HOME=$"test-bed/input.txt"
LOAN_ACCT_AUTO=$"test-bed/sample.txt"
LOAN_ACCT_PERSONAL_F1=$"test-bed/sample.txt"
LOAN_ACCT_PERSONAL_F2=$"test-bed/sample.txt"
MCLR_DATA=$"test-bed/mclr_data.txt"
PLR_DATA=$"test-bed/plr.txt"
REPAY_STRUCTURE=$"test-bed/repay.txt"
OUTPUT_FILE=$"test-bed/output.txt"
NPA=$"test-bed/npa.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--diagnostics-log-file  ${DIAGNOSTICS_FILE} \
--loan-acc-detail-auto ${LOAN_ACCT_AUTO} \
--loan-acc-detail-home ${LOAN_ACCT_HOME} \
--loan-acc-detail-personal-f1 ${LOAN_ACCT_PERSONAL_F1} \
--loan-acc-detail-personal-f2 ${LOAN_ACCT_PERSONAL_F2} \
--loan-repay-structure ${REPAY_STRUCTURE} \
--log-file ${LOG_FILE} \
--mclr-data-file ${MCLR_DATA} \
--npa-data-file ${NPA} \
--output-file ${OUTPUT_FILE} \
--plr-loan-acc-file ${PLR_DATA}
#--log-level debug \
#--diagnostics-flag false
