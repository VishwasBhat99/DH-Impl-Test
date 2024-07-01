#!/usr/bin/env bash

INPUT=$"test-bed/fin.cf"
OUTPUT=$"test-bed/op/"
LOG_FILE=$"test-bed/op/log.txt"
DIAGNOSTICS_FILE=$"test-bed/op/diag-log.txt"
METADATA_FILE=$"test-bed/cf-met.json"
REQ_FIELDS=$"test-bed/req_fields.json"
RULES_FILE=$"test-bed/rules_cf.txt"
SOURCE_MAP=$"test-bed/source_map_cf.txt"
DEFAULT=$"Default_cf.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--metadata-file-path $METADATA_FILE \
--req-fields-file-path $REQ_FIELDS \
--default-file-name $DEFAULT \
--source-map-file-path ${SOURCE_MAP} \
--rule-file-path ${RULES_FILE} \
--as-on-date 30-11-2022 \
--overdue-llg-code 112 \
--default-llg-code 8888 \
--dates-pos 0 \
#--log-level trace \
#--diagnostics-flag true
