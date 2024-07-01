#!/usr/bin/env bash

OUTPUT=$"test-bed/summary"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
LCR=$"test-bed/lcr_master.xlsx"
CONFIG_FILE=$"test-bed/config_lpa.json"

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file ${CONFIG_FILE} \
--lcr-master $LCR \
--as-on-date 22-10-2021 
#--log-level trace \
#--diagnostics-flag true
