#!/usr/bin/env bash

INPUT=$"test-bed/input.cf"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/req.json"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rules.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country "INDIA" \
--base-currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 31-01-2024 \
--is-overdue-req true \
--is-consolidated true \
--default-llg-code 8888 \
--amt-type PRIN \
--is-edate-req true \
--is-custom-bucket-req true \
--default-overdue-llg-path "test-bed/overdue_config.txt"
#--log-level trace \
#--diagnostics-flag true
