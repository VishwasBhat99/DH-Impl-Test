#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
RES_FILE=$"test-bed/repay.txt"
OP_FILE=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CALL_DATE=$"test-bed/call_date.txt"
REP=$"test-bed/rep_5.xlsx"

cargo run --release -- \
--input-file-path ${INP_FILE} \
--repayment-schedule-file-path ${RES_FILE} \
--output-file-path ${OP_FILE} \
--as-on-date "30-09-2020" \
--write-int-cashflows false  \
--next-rep-date-file ${REP} \
--od-additional-day 7 \
--log-file ${LOG_FILE} \
--rep-sheet-name "Sheet1" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
