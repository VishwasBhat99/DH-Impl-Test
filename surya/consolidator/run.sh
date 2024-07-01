#!/usr/bin/env bash

INPUT=$"test-bed/agg-out-td"
OUTPUT=$"test-bed/output/agg-out-td"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
CONSOL_CONFIG_FILE=$"test-bed/consol-config.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

rm -f test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--exchange-rate-file $CURRENCY_CONV_FILE \
--consol-config-file $CONSOL_CONFIG_FILE \
--log-file ${LOG_FILE} \
--is-maturity true \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2022 \
--input-files-type BALM
#--log-level trace \
#--diagnostics-flag true
