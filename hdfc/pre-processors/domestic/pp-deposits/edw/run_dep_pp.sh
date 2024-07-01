#!/usr/bin/env bash

INPUT=$"test-bed/sample.txt"
REF1=$"test-bed/R1.xlsx"
REF2=$"test-bed/Master_LLG_060322019.xlsx"
REF3=$"test-bed/R3.xlsx"
REF4=$"test-bed/currency_master.xlsx"
REF5=$"test-bed/sample.xlsx"
OUTPUT=$"test-bed/output.txt"
CONCAT=$"test-bed/concat.txt"
REC_OUTPUT=$"test-bed/TDReconRpt.txt"
FCNR=$"test-bed/sample.xlsx"
TOTAL_FD=$"test-bed/sample.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--ref-file-5 ${REF5} \
--total-fd-file ${TOTAL_FD} \
--fcnr-master-file ${FCNR} \
--rec-output-file ${REC_OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
