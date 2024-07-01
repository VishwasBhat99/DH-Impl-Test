#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/summary"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
KNOWN_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"
RULES_FILE=$"test-bed/rules.txt"
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency INR \
--local-consolidation-currency RUP \
--foreign-consolidation-currency FCY \
--exchange-rate-file $CURRENCY_CONV_FILE \
--known-fields-file $KNOWN_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-01-2019 \
--is-consolidated true \
--default-llg-code 9876 
#--log-level trace \
#--diagnostics-flag true
