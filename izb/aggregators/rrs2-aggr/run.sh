#!/usr/bin/env bash

INPUT=$"test-bed/RRSInputData.txt"
OUTPUT=$"test-bed/rrs2-aggregator-summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rules.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 28-02-2019 \
--default-llg-code 8888
#--log-level trace \
#--diagnostics-flag true
