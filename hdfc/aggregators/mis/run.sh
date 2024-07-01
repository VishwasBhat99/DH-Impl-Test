#!/usr/bin/env bash

INPUT=$"test-bed/loans/output.cf"
OUTPUT=$"test-bed/loans/aggregator/aggregated"
LOG_FILE=$"test-bed/loans/aggregator/log-from-generated.txt"
DIAGNOSTICS_FILE=$"test-bed/loans/aggregator/diagnostics-from-aggregator.txt"
CURRENCY_CONV_FILE=$"test-bed/loans/1000ExchangeRate.txt"
REQUIRED_FIELDS_FILE=$"test-bed/loans/req_fields.json"
METADATA_FILE=$"test-bed/loans/metadata.json"
RULES_FILE=$"test-bed/loans/rules.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--is-account-level-exchange-rate false \
--as-on-date 06-03-2019 \
--default-llg-code 9999 \
--log-level trace \
--diagnostics-flag true