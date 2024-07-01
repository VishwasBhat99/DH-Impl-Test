#!/usr/bin/env bash

INPUT=$"test-bed/hmm3020_28_feb.xls"
REF1=$"test-bed/Etrsry.xlsx"
REF2=$"test-bed/ORA_GL.xlsx"
REF3=$"test-bed/MIS1_Desc.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
OUTBOR=$"test-bed/bor.txt"
OUTLEN=$"test-bed/len.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/BNLFCYReconRpt.txt"
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
--concat-file ${CONCAT} \
--log-file ${LOG_FILE} \
--sheet-name Sheet1 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
