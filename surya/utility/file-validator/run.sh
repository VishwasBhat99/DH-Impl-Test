#!/usr/bin/env bash

CONFIG=$"test-bed/input_excel.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
INPUT_FILE=$"test-bed/29022024/input.txt"


cargo run -- \
--config-file ${CONFIG} \
--config-sheet-name 'Sheet1' \
--input-file-path ${INPUT_FILE} \
--as-on-date 29022024 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true
