#!/usr/bin/env bash

INPUT=$"test-bed/BucketDefinitionIn.txt"
REF1=$"test-bed/Overdue_LLG.xlsx"
REF2=$"test-bed/CBK_Liquidity_Report_Adj_KWD_30042022.xlsx"
REF3=$"test-bed/CBK_Liquidity_Report_Adj_FCY_30042022.xlsx"
BUC_DEF_OUT=$"test-bed/BucketDefinitionOut.txt"
OUTPUT=$"test-bed/output.xlsx"
OVRD_KWD_OUT=$"test-bed/output_ov_kwd.txt"
OVRD_FCY_OUT=$"test-bed/output_ov_fcy.txt"
BUC_MOC_KWD_OUT=$"test-bed/output_moc_kwd.txt"
BUC_MOC_FCY_OUT=$"test-bed/output_moc_fcy.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ovrd-llg ${REF1} \
--liq-rpt-kwd ${REF2} \
--liq-rpt-fcy ${REF3} \
--buc-def-out-file ${BUC_DEF_OUT} \
--ovrd-llg-sheet-name "Master" \
--liq-rpt-kwd-sheet-name "Master" \
--liq-rpt-fcy-sheet-name "Master" \
--output-file ${OUTPUT} \
--output-sheet-name "Sheet1" \
--ovrd-kwd-out ${OVRD_KWD_OUT} \
--ovrd-fcy-out ${OVRD_FCY_OUT} \
--buc-moc-kwd-out ${BUC_MOC_KWD_OUT} \
--buc-moc-fcy-out ${BUC_MOC_FCY_OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 01-02-2019 \
--log-level trace \
--diagnostics-flag true
