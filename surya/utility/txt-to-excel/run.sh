#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--output-sheet-name "Sheet1" \
--output-file ${OUTPUT_FILE} \
--as-on-date  28-02-2022 \
--date-formats %d-%h-%Y,%d-%b-%Y  \
--date-fields 7 \
--skip-header true \
--field-separator "," \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
