#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
CONFIG_FILE=$"test-bed/config.txt"
OUTPUT=$"test-bed/aggregated.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rule.txt"

cargo run --release -- \
--input-file ${INPUT} \
--config-file ${CONFIG_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country "INDIA" \
--base-currency INR \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 28-02-2023 \
--is-equally-distributed false \
--default-llg-code 8888 \
#--log-level trace \
#--diagnostics-flag true
