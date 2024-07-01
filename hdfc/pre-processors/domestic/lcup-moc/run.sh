#!/usr/bin/env bash

INPUT_FILE=$"test-bed/LCUPSKIP_INDIA_CE_06022023.xlsx"
MASTER1_FILE=$"test-bed/ORA_GL.xlsx"
MASTER2_FILE=$"test-bed/Master_LLG_Updated.xlsx"
OUTPUT=$"test-bed/MOC_LCUP_06022023.txt"
LOG_FILE=$"test-bed/lcup_moc_log.txt"
DIAGNOSTICS_FILE=$"test-bed/lcup_moc_diaglog.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT} \
--master1-file ${MASTER1_FILE} \
--master2-file ${MASTER2_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country "IND" \
--default-gl-code "10" \
--master1-sheet "Sheet1" \
--master2-sheet "Sheet1" \
--input-sheet "Sheet1" \
--as-on-date 06-02-2023
#--log-level trace \
#--diagnostics-flag true
