#!/usr/bin/env bash

INPUT=$"test-bed/cf-ubs-loans-output.cf"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rules.txt"

rm -f ../test-bed/aggregator/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--src-local-ccy OMR \
--display-local-ccy OMR \
--consol-ccy CON \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 28-02-2019 \
--is-consolidated true \
--account-level-exchange-rate false \
--default-llg-code 08888
#--log-level trace \
#--diagnostics-flag true
