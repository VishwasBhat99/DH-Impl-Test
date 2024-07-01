#!/usr/bin/env bash

INPUT=$"test-bed/new/aggregated"
OUTPUT=$"test-bed/new/output/aggregated"
CURRENCY_CONV_FILE=$"test-bed/new/1000ExchangeRate.txt"
CONSOL_CONFIG_FILE=$"test-bed/new/consol-config.txt"
LOG_FILE=$"test-bed/new/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/new/output/diag-log.txt"

rm -f ../test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--exchange-rate-file $CURRENCY_CONV_FILE \
--consol-config-file $CONSOL_CONFIG_FILE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--is-maturity true \
--as-on-date 27-01-2019
#--log-level trace \
#--diagnostics-flag true
