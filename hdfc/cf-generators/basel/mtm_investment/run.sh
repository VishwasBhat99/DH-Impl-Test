#!/usr/bin/env bash

INPUT=$"test-bed/CE_RR106_A02BD_YYYYMMDD.xlsx"
REF=$"test-bed/LMR_BOND_Master.xlsx"
OUT=$"test-bed/out"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run -- \
--input-file-path ${INPUT} \
--ref-file-path ${REF} \
--output-file-path ${OUT} \
--ccy INR \
--input-sheet-name "CE_RR106_A02BD_EXT_VW" \
--master-sheet-name "Sheet1" \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2020 \
--log-level trace \
--diagnostics-flag false 
