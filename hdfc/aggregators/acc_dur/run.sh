#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-ca.cf"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/ia_req_fields_ca.json"
METADATA_FILE=$"test-bed/metadata-casaod.json"
RULES_FILE=$"test-bed/rules.txt"

rm -f ../test-bed/aggregator/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--base-ccy INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-02-2019 \
--is-consolidated true \
--default-llg-code 08888 \
#--log-level trace \
#--diagnostics-flag true
