#!/usr/bin/env bash

BL_MASTER_FILE=$"test-bed/IB ""COUNTERPARTYWISE ""MM-POSITION ""SAS-ALM.csv"
BL_CF_FILE=$"test-bed/IB ""COUNTERPARTYWISE ""MM-POSITION ""SAS-ALM-CASHFLOW.csv"
BGL_CGL_FILE=$"test-bed/bgl_cgl.txt"
MAP_MASTER_FILE=$"test-bed/mapping_master.xlsx"
BL_OUTPUT_FILE=$"test-bed/bl_output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--bl-master-file "$BL_MASTER_FILE" \
--bl-cf-file "$BL_CF_FILE" \
--bgl-cgl-file ${BGL_CGL_FILE} \
--map-master-file ${MAP_MASTER_FILE} \
--sheet-name "LLG Master" \
--bl-output-file ${BL_OUTPUT_FILE} \
--date-fields "4|5|6" \
--header-rows "1" \
--delimeter-type "|" \
--as-on-date 31-01-2022 
#--log-level trace \
#--diagnostics-flag true
