#!/usr/bin/env bash

INPUT=$"test-bed/a.xlsx"
OUT=$"test-bed/output.txt"
PAR=$"test-bed/example.csv"
LOG_FILE=$"test-bed/log.txt"
EXC=$"test-bed/exc.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--base-ccy USD \
--is-consolidated-flag false \
--exchange-rate-file-path ${EXC} \
--input-sheet-name BLR-1_LCR \
--output-file ${OUT} \
--excel-config-file ${PAR} \
--subsidiary-id HSL \
--denomination 1000000 \
--currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-11-2019 \
--log-level trace \
--diagnostics-flag true \
