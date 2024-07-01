#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
#Input filename always expected to have _1 at the end.
INPUT_FILE1=$"test-bed/OAB_MappingMaster_1.xlsx"
INPUT_FILE2=$"test-bed/LineCodeMaster_2.xlsx"
APPEND_VAL="APPEND"

cargo run -- \
--input-file-1 ${INPUT_FILE1} \
--input-file-2 ${INPUT_FILE2} \
--sheet-name-1 "Sheet1" \
--sheet-name-2 "Sheet1" \
--as-on-date  31-01-2021 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
