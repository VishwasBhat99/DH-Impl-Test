#!/usr/bin/env bash

CONFIG_FILE=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"

cargo run --release -- \
--config-file ${CONFIG_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-09-2030 \
#--log-level trace \
#--diagnostics-flag true
