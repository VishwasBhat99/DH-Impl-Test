#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CONFIG_FILE=$"test-bed/config_manual.json"

cargo run -- \
--as-on-date 31-05-2023 \
--config-file-path ${CONFIG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--log-level trace \
--diagnostics-flag true
