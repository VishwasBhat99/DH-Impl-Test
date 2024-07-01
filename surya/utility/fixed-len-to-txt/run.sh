#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/TD-162_New.txt"
OUTPUT_FILE=$"test-bed/output.txt"
METADATA_FILE=$"test-bed/metadata.json"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--skip-header 9 \
--output-file ${OUTPUT_FILE} \
--metadata-file ${METADATA_FILE} \
--as-on-date  31-01-2021 \
--log-file ${LOG_FILE} \
--field-delimeter "|" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
