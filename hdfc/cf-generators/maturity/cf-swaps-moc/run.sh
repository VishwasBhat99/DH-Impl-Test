#!/usr/bin/env bash

BUCKET_CONFIG=$"test-bed/bkt_config.txt"
LOG_FILE=$"test-bed/log_file.txt"
DIAGNOSTICS_FILE=$"test-bed/diagnostic_log_file.txt"
INPUT_REC=$"test-bed/file_3.xlsx"
INPUT_PAY=$"test-bed/file_3.xlsx"
OUTPUT_FILE=$"test-bed/output"
CONFIG_FILE=$"test-bed/config_file.json"

cargo run --release -- \
--bucket-config-file ${BUCKET_CONFIG} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-rec-file ${INPUT_REC} \
--input-pay-file ${INPUT_PAY} \
--output-file ${OUTPUT_FILE} \
--config-file ${CONFIG_FILE} \
--as-on-date 31-12-2023 \
--skip-rows "1,2,3,5,11,12,13,15,21,22,23,25,31,32,33,35,41,42,43,44,46,52,53,54,56,62,63,64,66,72,73,74,76"
