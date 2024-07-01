#!/usr/bin/env bash

OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CONFIG_FILE=$"test-bed/ABconfig.json"
RULE_FILE=$"test-bed/rules.txt"
EXCHANGE_FILE=$"test-bed/exchg.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--config-file ${CONFIG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--exchange-rate-file ${EXCHANGE_FILE} \
--log-file-path ${LOG_FILE} \
--decimal-places 5 \
--output-file ${OUTPUT} \
# --negative-llgs "8888,12,13" \
# --abs-llgs "8888,21,22"
#--log-level trace \
#--diagnostics-flag true
