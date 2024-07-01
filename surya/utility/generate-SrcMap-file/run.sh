#!/usr/bin/env bash
OUTPUT_FILE=$"test-bed/output.txt"
INPUT_FILE=$"test-bed/example.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag.txt"

cargo run --release -- \
--output-file ${OUTPUT_FILE} \
--input-file-config ${INPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
