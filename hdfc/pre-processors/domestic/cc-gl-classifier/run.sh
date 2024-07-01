#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input"
MASTER_FILE=$"test-bed/master.xlsx"
OP_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
COUNTRY="IND"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--master-file-sheet-name "Sheet4" \
--country-name ${COUNTRY} \
--ccy "INR" \
--source-names "Vision Plus" \
--rf-llg "RF-LLG" \
--b1-llg "B1-LLG" \
--b2-llg "B2-LLG" \
--b3-llg "B3-LLG" \
--as-on-date 21-02-2022 \
--output-file ${OP_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
