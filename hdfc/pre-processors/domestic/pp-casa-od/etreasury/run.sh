#!/usr/bin/env bash

INPUT=$"test-bed/in.csv"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/MIS1_Desc.xlsx"
REF5=$"test-bed/NPA.txt"
REF6=$"test-bed/IndexCodeMaster.txt"
OUTPUT=$"test-bed/casaod.txt"
RECOUT=$"test-bed/UBSCASAODReconRpt.xt"
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
--alm-master-sheet-name "Master" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
