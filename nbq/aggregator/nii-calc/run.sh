#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
OUTPUT=$"test-bed/nii-out.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"

cargo run --release -- \
--config-file ${CONFIG} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2022
#--log-level trace \
#--diagnostics-flag true
