#!/usr/bin/env bash

INPUT=$"test-bed/cf_loan_finacle_output_performing_overude.cf"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/loans_finacle_dim_req_fields.json"
METADATA_FILE=$"test-bed/loan_finacle_metadata.json"
RULES_FILE=$"test-bed/loan_finacle_rules_overdue.txt"

rm -f ../test-bed/aggregator/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--src-local-ccy INR \
--display-local-ccy RUP \
--consol-ccy INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 30-11-2023 \
--is-consolidated false \
--account-level-exchange-rate false \
--default-llg-code 08888 \
--dim-id SEG \
--default-overdue-llg-path "test-bed/finacle_loans_overdue_config.txt" 
#--log-level trace \
#--diagnostics-flag true
