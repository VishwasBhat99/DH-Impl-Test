#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.xlsx"
OUTPUT_FILE=$"test-bed/output"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--sheet-name "Sheet1" \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-01-2021 \
--fields-pos-vec NA,4,2,3,1,1_4_3,"NOT GIVEN" \
--date-fields 1 \
--date-field-separator - \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
