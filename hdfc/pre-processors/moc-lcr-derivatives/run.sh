#!/usr/bin/env bash

INPUT_FILE=$"test-bed/test-case.xlsx"
CSA=$"test-bed/CSA.xlsx"
OUTPUT=$"test-bed/Derivatives_MOC.csv"
LOG_FILE=$"test-bed/log.txt"
EXCH=$"test-bed/exchange.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--exchange-rate-file ${EXCH} \
--output-file ${OUTPUT} \
--csa-file ${CSA} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency "INR" \
--country "IND" \
--input-sheet "Input" \
--csa-sheet "Sheet1" \
--as-on-date 30-01-2023
#--log-level trace \
#--diagnostics-flag true
