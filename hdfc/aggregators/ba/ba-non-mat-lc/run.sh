#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-invocation-lc.cf"
OUTPUT=$"test-bed/output/summary.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQUIRED_FIELDS_FILE=$"test-bed/req-fields-invocation.json"
METADATA_FILE=$"test-bed/metadata-invocation.json"
RULES_FILE=$"test-bed/rules-invocation.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency INR \
--local-consolidation-currency RUP \
--foreign-consolidation-currency FCY \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQUIRED_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-01-2019 \
--default-llg-code 08888 \
--is-consolidated false \
--log-level trace \
--diagnostics-flag true
