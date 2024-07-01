#!/usr/bin/env bash


INPUT=$"test-bed/cf_market_valuation.cf"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQUIRED_FIELDS_FILE=$"test-bed/vw_market_valuation_req_fields.json"
METADATA_FILE=$"test-bed/cf_market_valuation_metadata.json"
RULES_FILE=$"test-bed/market_valuation_rules.txt"


cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country IN \
--base-currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--is-acc-level-exrt false \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 30-09-2022 \
--default-llg-code 88888 \
--log-level trace \
--is-consolidated false \
--diagnostics-flag true
