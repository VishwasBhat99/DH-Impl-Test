#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/ALM_COA_Master.xlsx"
REF5=$"test-bed/MIS1_Desc.xlsx"
MF_MASTER=$"test-bed/sample.xlsx"
OUTPUT_CASA=$"test-bed/output-casa.txt"
CONCAT_OUTPUT_CASA=$"test-bed/concat-output-casa.txt"
CUST=$"test-bed/cust-master.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
    --input-file ${INPUT} \
    --ref-file-1 ${REF1} \
    --ref-file-2 ${REF2} \
    --ref-file-3 ${REF3} \
    --ref-file-4 ${REF4} \
    --ref-file-5 ${REF5} \
    --mf-master-file ${MF_MASTER} \
    --instance-name "casa" \
    --alm-master-sheet-name "Master" \
    --output-file-casa ${OUTPUT_CASA} \
    --output-concat-file-casa ${CONCAT_OUTPUT_CASA} \
    --cust-master-file ${CUST} \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --as-on-date 30-06-2022 \
    --sheet-name Sheet1 \
    --gl-type BH-Over-TD-CASA \
    --ref1-sheet-name "Sheet1" \
    --ref2-sheet-name "Sheet1" \
    --ref4-sheet-name "Sheet1" \
    --input-file-name BHRegulatory-CASA_Listing_Finance \
    --delimiter "|" \
    --log-level trace \
    --diagnostics-flag true
