#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/Output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/Output/diag-log.txt"

cargo run --release -- \
--config-file ${CONFIG} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2022 \
--log-level trace \
--bal-precision 4 \
--diagnostics-flag true
