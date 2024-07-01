#!/usr/bin/env bash

INPUT=$"test-bed/sample.txt"
REF1=$"test-bed/R1.xlsx"
REF2=$"test-bed/R2.xlsx"
REF3=$"test-bed/R3.xlsx"
REF4=$"test-bed/R4.xlsx"
REF5=$"test-bed/td_fin.txt"
OUTPUT=$"test-bed/output.txt"
CONCAT=$"test-bed/concat.txt"
REC_OUTPUT=$"test-bed/TDReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref-file-5 ${REF5} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${REC_OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
