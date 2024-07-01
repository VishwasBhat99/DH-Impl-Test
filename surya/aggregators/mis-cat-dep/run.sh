#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
RBI_CAT_DEF=$"test-bed/rbi_cat_dep.txt"
RBI_CAT_MAP=$"test-bed/rbi_cat_map.txt"
CUST_MASTER=$"test-bed/cust_master.txt"

cargo run --release -- \
--as-on-date 31-03-2020 \
--input-file-path ${INPUT} \
--cust-master-file-path ${CUST_MASTER} \
--cust-master-delimiter "|" \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--display-ccy "RUP" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--rbi-cat-def-file-path ${RBI_CAT_DEF} \
--rbi-cat-map-file-path ${RBI_CAT_MAP}
