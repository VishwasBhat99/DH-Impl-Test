#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/pp_gam1.json"
RULES_FILE=$"test-bed/rule.txt"
SOURCE_MAP=$"test-bed/source_map.txt"
DEFAULT=$"Default"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--metadata-file-path $METADATA_FILE \
--default-file-name $DEFAULT \
--source-map-file-path ${SOURCE_MAP} \
--rule-file-path ${RULES_FILE} \
--as-on-date 01-01-2019 \
--default-llg-code 8888 
#--log-level trace \
#--diagnostics-flag true
