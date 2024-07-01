#!/usr/bin/env bash

OUTPUT_FILE=$"test-bed/summary"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CONFIG_FILE=$"test-bed/config/config_master.json"

cargo run -- \
--output-file-path ${OUTPUT_FILE} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--config-file-path ${CONFIG_FILE} \
--as-on-date 01-01-2019 
#--log-level trace \
#--diagnostics-flag true
