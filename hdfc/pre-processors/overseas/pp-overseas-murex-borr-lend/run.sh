#!/usr/bin/env bash

INPUT=$"test-bed/money-market.txt"
REF1=$"test-bed/Murex_MM_Master.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/MIS1_Desc.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
REF5=$"test-bed/1000ExchangeRate.txt"
OUTBOR=$"test-bed/bor"
OUTLEN=$"test-bed/len"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/BNLReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
FUNDING_PATH=$"test-bed/Murex_MM_Master.xlsx"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ex-rt-file ${REF5} \
--concat-file ${CONCAT} \
--lcy INR \
--is-consolidated false \
--alm-master-sheet-name Master \
--output-file-borrowings ${OUTBOR} \
--output-file-lendings ${OUTLEN} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--sheet-name Sheet1 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 20-08-2019 \
--entity INDIA_CE \
--log-level trace \
--diagnostics-flag true \
--funding-source-file-path ${FUNDING_PATH} \
--funding-source-sheet-name Sheet1