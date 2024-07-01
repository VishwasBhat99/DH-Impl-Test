#!/usr/bin/env bash

INPUT=$"test-bed/input.xlsx"
CONFIG=$"test-bed/config.txt"
OUTPUT=$"test-bed/output.cf"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--as_on_date 15-05-2020 \
--days_range "1D,7D,14D,30D,2M,3M,6M,1Y,3Y,5Y,7Y,10Y,15Y" \
--input-file $INPUT \
--input-sheet-name "STL" \
--config-file $CONFIG \
--amt-col-pos 2 \
--output-file $OUTPUT \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
