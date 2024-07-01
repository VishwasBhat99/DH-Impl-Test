#!/usr/bin/env bash

INPUT=$"test-bed/NDS_OM_Data_112019.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--input-sheet-name "NDS OM Data_Nov 2019" \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--trade-source NDS \
--log-level debug 
# --diagnostics-flag true
