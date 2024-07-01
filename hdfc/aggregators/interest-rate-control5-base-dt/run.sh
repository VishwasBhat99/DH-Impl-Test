#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--config-file ${CONFIG} \
--as-on-date 12-06-2021 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true
