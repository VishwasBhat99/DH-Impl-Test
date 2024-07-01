#!/usr/bin/env bash

INPUT=$"test-bed/input.csv"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/Ora_PROD.xlsx"
REF3=$"test-bed/Ora_GL.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
OUTPUT=$"test-bed/output.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/SecInvstReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  --release -- \
--input-file ${INPUT} \
--ora-gl-file ${REF3} \
--alm-master-file ${REF4} \
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019  
