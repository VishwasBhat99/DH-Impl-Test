#!/usr/bin/env bash

INPUT_FILE=$"test-bed/ALM_SURYA_CREDENCE_31072023.txt"
TCFSL_FILE=$"test-bed/TCFSL.ods"
INVESTMENT_FEATURE_FILE=$"test-bed/Investment_Future_JUN-23.xlsx"
OUTPUT_FILE=$"test-bed/td_output.txt"
LOG_FILE=$"test-bed/log.txt"
CREDENCE_MAPPING_MASTER_FILE=$"test-bed/Credence_Mapping_Master.xlsx"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
ALM_CREDENCE=$"test-bed/Credence_Manual.xlsx"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--investment-future-file ${INVESTMENT_FEATURE_FILE} \
--tcfsl-sheet-name "Stage III" \
--investment-future-sheet-name "Credence-CF" \
--tcfsl-file  ${TCFSL_FILE} \
--alm-credence-manual-file ${ALM_CREDENCE} \
--alm-credence-sheet-name "OPS REMARK" \
--output-file ${OUTPUT_FILE} \
--cred-gl-mapping-master-sheet-name Sheet1 \
--cred-gl-mapping-master-file ${CREDENCE_MAPPING_MASTER_FILE} \
--as-on-date 31-01-2024
# --log-level trace \
# --diagnostics-flag true
