#!/usr/bin/env bash

INPUT=$"test-bed/pp_gl_output.cf"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQUIRED_FIELDS_FILE=$"test-bed/gl_req_fields.json"
METADATA_FILE=$"test-bed/gl_metadata.json"
RULES_FILE=$"test-bed/gl_rules.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--src-local-ccy INR \
--display-local-ccy INR \
--consol-ccy RUP \
--is-amt-abs true \
--is-aggr-amt-abs true \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 30-09-2021 \
--default-llg-code 08888 \
--is-consolidated false \
--log-level trace \
--diagnostics-flag true
