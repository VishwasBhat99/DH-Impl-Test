#!/usr/bin/env bash

CONFIG=$"test-bed/config/config_master.json"
OUTPUT=$"test-bed/config/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
ACCOUNTS_MATURED=$"test-bed/matured_accounts.txt"


cargo run -- \
--config-file ${CONFIG} \
--output-file ${OUTPUT} \
--matured-accounts-file ${ACCOUNTS_MATURED} \
--as-on-date 12-04-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true
