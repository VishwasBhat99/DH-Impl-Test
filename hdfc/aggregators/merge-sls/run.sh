#!/usr/bin/env bash

CONFIG=$"test-bed/input-resources/input.json"
LOG_FILE=$"test-bed/output/log/int_aggr_borr_log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/log/int_aggr_borr_diag_log.txt"

cargo run --release -- \
--config-file-path ${CONFIG} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2020 \
--log-level trace \
--diagnostics-flag false 
