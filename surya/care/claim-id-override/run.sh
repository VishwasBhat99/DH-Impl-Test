#!/usr/bin/env bash
INPUT_CONFIG=$"test-bed/config.json"
RULES_FILE=$"test-bed/rules.txt"
OUTPUT_FILE=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/dialog.txt"

cargo run --release -- \
--input-config-file ${INPUT_CONFIG} \
--output-file-path ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
