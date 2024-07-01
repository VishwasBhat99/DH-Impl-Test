#!/usr/bin/env bash

CONFIG_FILE=$"test-bed/config-file.json"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file ${CONFIG_FILE} \
--as-on-date 30-11-2022 \
--output-file ${OUTPUT_FILE}
#--log-level trace \
#--diagnostics-flag true
