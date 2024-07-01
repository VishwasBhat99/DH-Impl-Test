#!/usr/bin/env bash

INPUT=$"test-bed/input.cf"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/cf_metadata.json"
RULES_FILE=$"test-bed/cf_rules.txt"
LLG_OP=$"test-bed/llg_map.txt"


cargo run --release -- \
--input-file ${INPUT} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date $2 \
--llg-op-mapping-file ${LLG_OP} \
--default-llg-code 08888
#--log-level trace \
#--diagnostics-flag true
