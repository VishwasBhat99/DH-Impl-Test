#!/usr/bin/env bash

INPUT=$"test-bed/Book2.xlsx"
OUTPUT=$"test-bed/CR_OFfBal.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
SHEET_NAME=$"CR_OFfBal"

target/release/cr_bal_moc \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--base-currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--sheet-name $SHEET_NAME \
--exchange-rate-file $CURRENCY_CONV_FILE \
--as-on-date 30-09-2020
#--log-level trace \
#--diagnostics-flag true
