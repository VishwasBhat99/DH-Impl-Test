#!/usr/bin/env bash

REMOTE_FOLDER=$"/home/surya/pawan/NewResV2"
ACOUNT_CONFIG=$"test-bed/account_config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--remote-folder-path ${REMOTE_FOLDER} \
--account-config-file ${ACOUNT_CONFIG} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false

