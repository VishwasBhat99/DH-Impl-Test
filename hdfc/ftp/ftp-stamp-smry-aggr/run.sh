#!/usr/bin/env bash

INPUT=$"test-bed/CFOutput_add.cf"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
KNOWN_FIELDS_FILE=$"test-bed/req-fields.json"
METADATA_FILE=$"test-bed/add_metadata_out.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--known-fields-file $KNOWN_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--as-on-date 24-01-2011 \
#--log-level trace \
#--diagnostics-flag true
