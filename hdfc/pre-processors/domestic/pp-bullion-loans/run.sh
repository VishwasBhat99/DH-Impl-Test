#!/usr/bin/env bash

INPUT=$"test-bed/OutstandingLoan_20190222.csv"
REF1=$"test-bed/GL_ALM_Mapping.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/BullLoansReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--alm-master-sheet-name "GL_ALM_Mapping" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--currency USD \
--log-level trace \
--diagnostics-flag true
