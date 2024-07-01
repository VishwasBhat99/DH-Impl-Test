#!/usr/bin/env bash

INPUT=$"test-bed/input.xlsx"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/ALM_COA_Master.xlsx"
REF5=$"test-bed/MIS1_Desc.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/BHOverBillsReconRpt.txt"
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
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--sheet-name Sheet1 \
--gl-type BH-Over-Bills \
--input-file-name BH_Outstanding_Bills \
--log-level trace \
--diagnostics-flag true
