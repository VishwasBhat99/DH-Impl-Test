#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
PRODUCT_RPT_FILE=$"test-bed/Book1.xlsx"
LLG_MAPPING_FILE=$"test-bed/llg_mapping.txt"
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--product-rpt-file ${PRODUCT_RPT_FILE} \
--llg-mapping-file ${LLG_MAPPING_FILE} \
--exchange-rate-file $CURRENCY_CONV_FILE
#--log-level trace \
#--diagnostics-flag true
