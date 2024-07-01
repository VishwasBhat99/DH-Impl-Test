#!/usr/bin/env bash

INPUT_FILE=$"test-bed/ALM_SURYA_BANCS_31072023.txt"
TCFSL_FILE=$"test-bed/TCFSL.ods"
REPAYMENTSCHEDULE_FILE=$"test-bed/ALM_SURYA_BANCS_REPAYMENT_31072023.txt"
PRODUCTENTITYMAPPING_FILE=$"test-bed/ProductEntityMapping.csv"
PRODUCT_ID_FILE=$"test-bed/PROD_ID.txt"
PRODUCT_FIXED_FLOATING_FILE=$"test-bed/PRODUCT_FIXED_FLOATING.csv"
WRITEOFF_FILE=$"test-bed/writeoff_merged.txt"
OUTPUT_FILE=$"test-bed/td_output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--writeoff-file ${WRITEOFF_FILE} \
--input-file ${INPUT_FILE} \
--repayment-schedule-file ${REPAYMENTSCHEDULE_FILE} \
--product-entity-mapping-file ${PRODUCTENTITYMAPPING_FILE} \
--product-id-file ${PRODUCT_ID_FILE} \
--product-fixed-floating-file ${PRODUCT_FIXED_FLOATING_FILE} \
--tcfsl-sheet-name "Stage III" \
--int-basis "INT" \
--tcfsl-file ${TCFSL_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date 30-10-2001 \
--log-level trace \
--diagnostics-flag true
