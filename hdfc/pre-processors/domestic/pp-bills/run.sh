#!/usr/bin/env bash

INPUT=$"test-bed/input.csv"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/MIS1_Desc.xlsx"
REF5=$"test-bed/INP001_NPA.txt"
OUTPUT=$"test-bed/out.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/BillsReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
WEAKER_MASTER=$"test-bed/Weaker_sec_master.xlsx"
EWS_MASTER=$"test-bed/EWS_weaker_master.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref-file-5 ${REF5} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--concat-file ${CONCAT} \
--weaker-sec-master ${WEAKER_MASTER} \
--ews-weaker-master ${EWS_MASTER} \
--weaker-sec-sheet-name "Sheet1" \
--ews-weaker-sheet-name "Sheet1" \
--as-on-date 27-01-2019
