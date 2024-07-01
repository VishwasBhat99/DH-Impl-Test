#!/usr/bin/env bash

INPUT=$"test-bed/cf-ubs-loans-output.cf"
REF1=$"test-bed/spread_ref.xlsx"
OUTPUT=$"test-bed/aggregated"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/req-fields.json"
METADATA_FILE=$"test-bed/a_w_cf_ubs_loans.json"
RULES_FILE=$"test-bed/rules.txt"
TENOR_FILE=$"test-bed/tenor.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file ${REF1} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--tenor-file ${TENOR_FILE} \
--src-local-ccy INR \
--display-local-ccy RUP \
--consol-ccy INR \
--foreign-consol-ccy FCY \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 31-05-2019 \
--account-level-exchange-rate false \
--default-llg-code 08888 \
--npa-flag-values "DOUBTFUL I,Doubtful II,LOSS,Substandard,DBT I,DBT II,DBT III " \
--is-consolidated false \
--log-level trace \
--diagnostics-flag true
