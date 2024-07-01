#!/usr/bin/env bash

INPUT=$"test-bed/sample_rep_dt.txt"
SMA_FILE=$"test-bed/sma.csv"
REF1=$"test-bed/almmaster.xlsx"
REF2=$"test-bed/costcentre.xlsx"
REF3=$"test-bed/Finone_Rate_Code_Master.xlsx"
REF4=$"test-bed/INP001_NPA.txt"
REF5=$"test-bed/Spread_Org.xlsx"
REF6=$"test-bed/NPA_Master.txt"
REF7=$"test-bed/FN_Interest_Income.txt"
REF8=$"test-bed/FN_Income_GL_Master.xlsx"
MAPPER_FILE=$"test-bed/mapper_col.txt"
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
--ref-file-6 ${REF6} \
--ref-file-7 ${REF7} \
--ref-file-8 ${REF8} \
--mapper-col-file ${MAPPER_FILE} \
--alm-master-sheet-name "Sheet1" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${REC_OUTPUT} \
--log-file ${LOG_FILE} \
--data-src-name "FINNONE" \
--sma-file-path ${SMA_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-04-2019 \
#--log-level trace \
#--diagnostics-flag true
