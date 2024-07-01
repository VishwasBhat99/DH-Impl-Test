#!/usr/bin/env bash

DBCONFIG=$"test-bed/db-config.txt"
RULES_FILE=$"test-bed/"
INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/"
METADATA_FILE=$"test-bed/metadata.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--as-on-date "01-04-2021" \
--dbconfig-file ${DBCONFIG} \
--rules-output-path ${RULES_FILE} \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--default-llg-code 9999 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
