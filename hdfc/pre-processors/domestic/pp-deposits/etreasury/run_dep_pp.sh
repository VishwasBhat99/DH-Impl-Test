#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REF1=$"test-bed/R1.xlsx"
REF2=$"test-bed/Master_LLG_060322019.xlsx"
REF3=$"test-bed/R3.xlsx"
OUTPUT=$"test-bed/output.txt"
REC_OUTPUT=$"test-bed/TDReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--rec-output-file ${REC_OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019
#--log-level trace \
#--diagnostics-flag true
