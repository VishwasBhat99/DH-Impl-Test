#!/usr/bin/env bash

INPUT=$"test-bed/test-bed/abc.cf"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
KNOWN_FIELDS_FILE=$"test-bed/test-bed/req_field.json"
METADATA_FILE=$"test-bed/test-bed/od-metadata.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--known-fields-file $KNOWN_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--as-on-date $1 \
#--log-level trace \
#--diagnostics-flag true
