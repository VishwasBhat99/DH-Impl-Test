#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REF1=$"test-bed/Ora_PROD.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/ALM_COA_Master.xlsx"
REF5=$"test-bed/MIS1_Desc.xlsx"
MF_MASTER=$"test-bed/sample.xlsx"
OUTPUT_TD=$"test-bed/output-td.txt"
OUTPUT_CASA=$"test-bed/output-casa.txt"
OUTPUT_OD=$"test-bed/output-od.txt"
CONCAT_OUTPUT_TD=$"test-bed/concat-output-td.txt"
CONCAT_OUTPUT_CASA=$"test-bed/concat-output-casa.txt"
CONCAT_OUTPUT_OD=$"test-bed/concat-output-od.txt"
CUST=$"test-bed/cust-master.txt"
RECOUT=$"test-bed/BHReconRpt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
    --input-file ${INPUT} \
    --ref-file-1 ${REF1} \
    --ref-file-2 ${REF2} \
    --ref-file-3 ${REF3} \
    --ref-file-4 ${REF4} \
    --ref-file-5 ${REF5} \
    --mf-master-file ${MF_MASTER} \
    --instance-name "" \
    --alm-master-sheet-name "Master" \
    --output-file-td ${OUTPUT_TD} \
    --output-file-od ${OUTPUT_OD} \
    --output-file-casa ${OUTPUT_CASA} \
    --output-concat-file-td ${CONCAT_OUTPUT_TD} \
    --output-concat-file-od ${CONCAT_OUTPUT_OD} \
    --output-concat-file-casa ${CONCAT_OUTPUT_CASA} \
    --cust-master-file ${CUST} \
    --rec-output-file ${RECOUT} \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --as-on-date 27-01-2019 \
    --sheet-name Sheet1 \
    --gl-type BH-Over-TD-CASA \
    --input-file-name BHRegulatory-CASA_Listing_Finance \
    --log-level trace \
    --diagnostics-flag true
