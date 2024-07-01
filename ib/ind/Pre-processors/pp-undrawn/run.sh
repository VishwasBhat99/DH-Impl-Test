#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
MAP_MASTER_FILE=$"test-bed/IB-MappingMaster.xlsx"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--mapping-master-file ${MAP_MASTER_FILE} \
--mapping-sheet-name "LLG Master" \
--output-file ${OUTPUT_FILE} \
--as-on-date 31-12-2022
#--log-level trace \
#--diagnostics-flag true
