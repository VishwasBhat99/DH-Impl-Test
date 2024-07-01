#!/usr/bin/env bash

INP=$"test-bed/Annex - MA(BS)1E 04 Dec 2020 (3).xlsx"
EXRT=$"test-bed/1000ExchangeRate.txt"
OUT=$"test-bed/out.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--input-file-path "${INP}" \
--exrt-file-path ${EXRT} \
--log-file-path ${LOG_FILE} \
--output-file-path ${OUT} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-19 \
--log-level trace \
--input-sheet-name "Report" \
--to-ccy "USD" \
--from-ccy "HKD" \
--output-sheet-name "Report" \
#--is-perf-diagnostics-enabled false 

