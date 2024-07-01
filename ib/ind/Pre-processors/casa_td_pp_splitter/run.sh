#!/usr/bin/env bash

BASE_INPUT_FILE=$"test-bed/base_input.txt"
CUST_MASTER_FILE=$"test-bed/customer_master.txt"
MAP_MASTER_FILE=$"test-bed/mapping_master.xlsx"
TD_IDENTIFIER_FILE=$"test-bed/td_identifier.txt"
SB_INTRATE_FILE=$"test-bed/SB_Int_Rate.txt"
TD_OUTPUT_FILE=$"test-bed/td_output.txt"
CASA_OUTPUT_FILE=$"test-bed/casa_output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--base-input-file ${BASE_INPUT_FILE} \
--cust-master-file ${CUST_MASTER_FILE} \
--map-master-file ${MAP_MASTER_FILE} \
--td-identifier-file ${TD_IDENTIFIER_FILE} \
--sb-intrate-file ${SB_INTRATE_FILE} \
--td-output-file ${TD_OUTPUT_FILE} \
--casa-output-file ${CASA_OUTPUT_FILE} \
--splitter-field "1" \
--sheet-name "LLG Master" \
--date-fields "14|15|16|20|22|23|30|31" \
--header-rows "1" \
--delimeter-type "|" \
--as-on-date 31-01-2022 
#--log-level trace \
#--diagnostics-flag true
