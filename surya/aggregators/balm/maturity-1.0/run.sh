#!/usr/bin/env bash

INPUT=$"test-bed/out.cf"
OUTPUT=$"test-bed/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/ex.txt"
REQ_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/req_out.json"
RULES_FILE=$"test-bed/rules.txt"
OVERDUE_CONFIG=$""

rm -f ../test-bed/aggregator/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--src-local-ccy INR \
--display-local-ccy INR \
--consol-ccy INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 15-02-2024 \
--is-consolidated false \
--account-level-exchange-rate false \
--default-llg-code 08888 \
--default-overdue-llg-path ${OVERDUE_CONFIG}
#--log-level trace \
#--diagnostics-flag true
