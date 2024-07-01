#!/usr/bin/env bash

INVESTMENT_FILE=$"test-bed/input.csv"
BGL_CGL_FILE=$"test-bed/bgl_cgl.txt"
MAP_MASTER_FILE=$"test-bed/mapping_master.xlsx"
INVEST_OUTPUT_FILE=$"test-bed/investment_output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--investment-file ${INVESTMENT_FILE} \
--bgl-cgl-file ${BGL_CGL_FILE} \
--map-master-file ${MAP_MASTER_FILE} \
--sheet-name "LLG Master" \
--investment-output-file ${INVEST_OUTPUT_FILE} \
--date-fields "11|53|54|55|56|81|82|85|86|118|119" \
--instrument-type-data "" \
--header-rows "1" \
--delimeter-type "|" \
--currency "INR" \
--as-on-date 31-01-2022 
#--log-level trace \
#--diagnostics-flag true
