#!/usr/bin/env bash

INPUT=$"test-bed/input.csv"
CUSTMASTER=$"test-bed/cm.csv"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/ALM_COA_Master.xlsx"
REF5=$"test-bed/MIS1_Desc.xlsx"
OUTPUT_TD=$"test-bed/output-td.txt"
OUTPUT_CASA=$"test-bed/output-casa.txt"
RECOUT=$"test-bed/BHReconRpt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref-file-5 ${REF5} \
--alm-master-sheet-name "Master" \
--output-file-td ${OUTPUT_TD} \
--output-file-casa ${OUTPUT_CASA} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--gl-type BH-Over-TD-CASA \
--input-file-name BHRegulatory-CASA_Listing_Finance \
--cust-master-file ${CUSTMASTER} \
--log-level trace \
--diagnostics-flag true
