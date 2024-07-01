#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/exrt.txt"
REQ_FIELDS_FILE=$"test-bed/loans_req_fields.json"
METADATA_FILE=$"test-bed/metadata-gstt.json"
RULES_FILE=$"test-bed/rules-gstt.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country "IND" \
--is-amt-abs true \
--base-currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-01-2019 \
--is-consolidated false \
--default-llg-code 8888 \
--bucket-id 234 \
#--log-level trace \
#--diagnostics-flag true
