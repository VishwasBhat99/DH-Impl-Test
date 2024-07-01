#!/usr/bin/env bash

INPUT=$"test-bed/Inv_sample.xls"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/Etrsry.xlsx"
REF5=$"test-bed/HFT-SLR_Defeasance_2019-05_Fincon.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/InvstReconRpt.txt"
CONCAT=$"test-bed/concat.txt"
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
--ref-file-5-sheet-name "GL SDL" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--sheet-name Sheet1 \
--log-level trace \
--diagnostics-flag true
