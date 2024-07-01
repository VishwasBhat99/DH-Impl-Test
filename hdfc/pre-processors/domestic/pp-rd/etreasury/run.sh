#!/usr/bin/env bash

INPUT=$"test-bed/rdclient.txt"
REF1=$"test-bed/RD_GLMapping_Master.xlsx"
REF2=$"test-bed/GL_ALM_Mapping.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/RDReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--alm-master-sheet-name "GL_ALM_Mapping" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
