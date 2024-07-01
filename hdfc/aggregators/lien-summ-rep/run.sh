#!/usr/bin/env bash

OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CONFIG_FILE=$"test-bed/config.json"

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file ${CONFIG_FILE} \
--country-code IND \
--as-on-date 30-07-2021 \
