#!/usr/bin/env bash

INPUT_FILE=$"test-bed/Contingent_liability_ALM.xlsx"
OUTPUT=$"test-bed/pp-cont-output.txt"
LOG_FILE=$"test-bed/cf-ubs-loans-log.txt"
CONFIG_FILE=$"test-bed/config.json"
DIAGNOSTICS_FILE=$"test-bed/cf-ubs-loans-diag-log.txt"

cargo run -- \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--input-master-file ${INPUT_FILE} \
--as-on-date 31-01-2024 \
--config-file-path ${CONFIG_FILE} \
--input-master-sheet-name "Summary" \
--display-ccy "INR" \

