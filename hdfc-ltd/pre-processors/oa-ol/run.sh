#!/usr/bin/env bash

INPUT=$"test-bed/input.xlsx"
OUTPUT=$"test-bed/Bills.txt"
RECOUT=$"test-bed/BillsReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--sheet-name "Sheet1" \
--output-file-path ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag flase
