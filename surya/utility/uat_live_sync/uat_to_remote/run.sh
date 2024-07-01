#!/usr/bin/env bash

REMOTE_FOLDER=$"/home/surya/pawan/NewResV2"
ACOUNT_CONFIG=$"test-bed/account_config.json"
ALL_STREAMS=$"test-bed/all_streams/"
BATCH_INFO=$"test-bed/batch_info.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--remote-folder-path ${REMOTE_FOLDER} \
--account-config-file ${ACOUNT_CONFIG} \
--all-streams ${ALL_STREAMS} \
--batch-info-file ${BATCH_INFO} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
