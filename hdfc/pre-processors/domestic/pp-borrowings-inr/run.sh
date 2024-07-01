#!/usr/bin/env bash

INPUT=$"test-bed/hmm3011.xls"
REF1=$"test-bed/Etrsry.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/MIS1_Desc.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
OUTBOR=$"test-bed/bor.txt"
OUTLEN=$"test-bed/len.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/BORR_LEND_INRReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--alm-master-sheet-name "Master" \
--output-file-borrowings ${OUTBOR} \
--output-file-lendings ${OUTLEN} \
--rec-output-file ${RECOUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--sheet-name Sheet1 \
--log-level trace \
--diagnostics-flag true
