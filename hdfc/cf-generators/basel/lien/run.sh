#!/usr/bin/env bash

INPUT=$"test-bed/lien_test.xlsx"
OUTPUT=$"test-bed/bb"
CURR_FILE=$"test-bed/1000ExchangeRate.txt"
CUST_MASTER=$"test-bed/cust-master.txt"
LOG=$"test-bed/log.txt"
DIAG_LOG=$"test-bed/diag-log.txt"

cargo run --release -- \
--acc-id-col-id 0 \
--act-ccy-col-id 6 \
--as-on-date 08-01-2020 \
--base-currency INR \
--country IND \
--cust-typ-col-id 0 \
--cust-type-ref-path $CUST_MASTER \
--diagnostics-log-file $DIAG_LOG \
--exchange-rate-file $CURR_FILE \
--exp-date-col-id 7 \
--fd-amt-col-id 1 \
--input-currency INR \
--input-file $INPUT \
--input-sheet-name "Sheet1" \
--log-file $LOG \
--os-col-id 3 \
--output-file $OUTPUT
