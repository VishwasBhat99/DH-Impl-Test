#!/usr/bin/env bash

INPUT=$"../test-bed/CFOutput.cf"
OUTPUT=$"../test-bed/aggregator/aggregated"
LOG_FILE=$"../test-bed/aggregator/log-from-generated.txt"
DIAGNOSTICS_FILE=$"../test-bed/aggregator/diagnostics-from-aggregator.txt"
CURRENCY_CONV_FILE=$"../test-input-resources/1000ExchangeRate.txt"
KNOWN_FIELDS_FILE=$"../test-input-resources/knownfields.txt"
METADATA_FILE=$"../test-bed/metadata.json"
RULES_FILE=$"../test-bed/rules.txt"
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--known-fields-file $KNOWN_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 2018-02-28
#--log-level trace \
#--diagnostics-flag true