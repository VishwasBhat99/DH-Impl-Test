#!/usr/bin/env bash

CONFIG_FILE=$"test-bed/config.json"
MASTER_FILE=$"test-bed/master.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--config-file ${CONFIG_FILE} \
--master-file ${MASTER_FILE} \
--master-file-delimiter "|" \
--as-on-date 27-01-2019 \
--log-level trace \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--diagnostics-flag true
