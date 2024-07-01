#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CONFIG_FILE=$"test-bed/config.json"
PP_MERGER_OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--config-file ${CONFIG_FILE} \
--output-file ${PP_MERGER_OUTPUT_FILE} \
--as-on-date  30-04-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
