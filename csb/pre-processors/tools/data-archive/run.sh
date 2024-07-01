#!/usr/bin/env bash

INPUT_PATH=$"test-bed/input/"
PREPROCESS=$"test-bed/preprocess/"
CFDATA=$"test-bed/cfdata/"
SUMMARY=$"test-bed/summary/"
LOGS=$"test-bed/logs/"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
ZIP_PATH=$"test-bed/"

cargo run --release -- \
--input-file-path ${INPUT_PATH} \
--zip-file-path ${ZIP_PATH} \
--preprocess-path ${PREPROCESS} \
--cfdata-path ${CFDATA} \
--summary-path ${SUMMARY} \
--logs-path ${LOGS} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--as-on-date "30-06-2021" \
--date-format "DDMMYYYY" \
--diagnostics-flag true 
