#!/usr/bin/env bash

INPUT=$"test-bed/CFOutput.cf"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REQUIRED_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--as-on-date 06-03-2019 \
--from-date 06-03-1970 \
--to-date 06-03-2999 \
--threshold-bal 0.0 \
--log-level trace \
--diagnostics-flag true
