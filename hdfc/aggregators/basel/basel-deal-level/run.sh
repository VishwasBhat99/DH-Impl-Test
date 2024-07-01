#!/usr/bin/env bash

INPUT=$"test-bed/input.cf"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQUIRED_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rules.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country IN \
--base-currency OMR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-01-2019 \
--default-llg-code 08888 \
--log-level trace \
--diagnostics-flag true
