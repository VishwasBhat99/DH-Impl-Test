#!/usr/bin/env bash

INPUT=$"test-bed/ALM-Term-Loan-Master.txt"
REF2=$"test-bed/ALM-Cashflow-Term-Loans.txt"
REF3=$"test-bed/Benpos-Data.xlsx"
REF4=$"test-bed/Benpos-Mapping-Master.xlsx"
REF5=$"test-bed/TermLoan-Updatetype-Master.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref3-sheet-name "Input" \
--ref4-sheet-name "Sheet1" \
--ref-file-5 ${REF5} \
--ref5-sheet-name "Sheet1" \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
