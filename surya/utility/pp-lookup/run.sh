#!/usr/bin/env bash

LOG_FILE=$"test-bed/Output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/Output/diag-log.txt"
CONFIG_FILE=$"test-bed/config.json"

cargo run --release -- \
--config-file ${CONFIG_FILE} \
--as-on-date  31-03-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
