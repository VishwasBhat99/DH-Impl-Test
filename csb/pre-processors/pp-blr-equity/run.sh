#!/usr/bin/env bash

INPUT=$"test-bed/ADR.txt"
OUT=$"test-bed/output.csv"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXCHNAGE_RATE=$"test-bed/USD_Maker_Check_Rate.xlsx"
FIELDS_FILE=$"fields_config.json"
SYMBOL=$"EQ"
FACE_VALUE=$"1"
./target/release/pp_blr05_equity \
--input-file ${INPUT} \
--output-file ${OUT} \
--fields-file ${FIELDS_FILE} \
--symbol ${SYMBOL} \
--face-value ${FACE_VALUE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-04-2020 \
--exchange-rate-file ${EXCHNAGE_RATE} \
--log-level trace \
--diagnostics-flag true \
