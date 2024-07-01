#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--config-file-path ${CONFIG} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-19 \
--log-level trace \
--diagnostics-flag false 
