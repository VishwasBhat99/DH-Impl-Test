#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--config-file-path ${CONFIG} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 30-09-2020 \
--log-level trace \
--top-cust-count 20 \
--diagnostics-flag false 
