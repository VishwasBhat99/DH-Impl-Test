#!/usr/bin/env bash

INPUT=$"test-bed/gl/GLAggregated"
OUTPUT=$"test-bed/output/GLAggregated-threshold"
LOG_FILE=$"test-bed/gl/log.txt"
DIAGNOSTICS_FILE=$"test-bed/gl/diaglog.txt"
PRODUCT_RPT_FILE=$"test-bed/Threshold_Master.xlsx"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--product-rpt-file ${PRODUCT_RPT_FILE} \
--exchange-rate-file $CURRENCY_CONV_FILE \
--base-ccy "INR" \
--calc-type "ABSOLUTE-PERCENTAGE" \
--calc-percentage 80.00 \
#--log-level trace \
#--diagnostics-flag true