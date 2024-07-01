#!/usr/bin/env bash

INPUT_FILE=$"test-bed/sample.xml"
OUTPUT_FILE=$"test-bed/Output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false \
--is-header-present true
