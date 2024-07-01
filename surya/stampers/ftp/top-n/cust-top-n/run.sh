#!/usr/bin/env bash
CONFIG=$"test-bed/ConfigFile.json"
OUTPUT=$"test-bed/output.txt"
CUSTDEF=$"test-bed/cust_def.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--as-on-date 31-07-2023 \
--config-file ${CONFIG} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file-path ${LOG_FILE} \
--output-file ${OUTPUT} \
--cust-def-file ${CUSTDEF} \
--cust-id-position 2 \
--cust-name-position 3 \
#--log-level trace \
#--diagnostics-flag true
