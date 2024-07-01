#!/usr/bin/env bash

INPUT=$"test-bed/input.csv"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/Ora_PROD.xlsx"
REF3=$"test-bed/Ora_GL.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/SecLoansReconRpt.txt"
CONCAT=$"test-bed/concat.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--concat-file ${CONCAT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
