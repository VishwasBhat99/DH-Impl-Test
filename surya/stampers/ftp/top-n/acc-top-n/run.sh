#!/usr/bin/env bash
CONFIG=$"test-bed/ConfigFile.json"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
INPUT=$"test-bed/top-cust-def.txt"


cargo run -- \
--top-n-input-file ${INPUT} \
--tot-field-number 60 \
--as-on-date 31-07-2023 \
--config-file ${CONFIG} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file-path ${LOG_FILE} \
--output-file ${OUTPUT} \
#--log-level trace \
#--diagnostics-flag true