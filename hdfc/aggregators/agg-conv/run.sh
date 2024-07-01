#!/usr/bin/env bash

INPUT=$"test-bed/aggregated"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--exchange-rate-file $CURRENCY_CONV_FILE \
--base-currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 19-08-2019
#--log-level trace \
#--diagnostics-flag true
