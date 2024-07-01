#!/usr/bin/env bash
CONFIG=$"test-bed/ABConfig.json"
OUTPUT_FILE=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"


cargo run --release -- \
--config-file ${CONFIG} \
--output-file ${OUTPUT_FILE} \
--as-on-date  30-04-2023 \
--log-file ${LOG_FILE} \
--output-sheet-name "Sheet8" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false