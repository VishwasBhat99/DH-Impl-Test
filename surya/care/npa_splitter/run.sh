#!/usr/bin/env bash
CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--config-file-path ${CONFIG} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
