#!/usr/bin/env bash

PARAM=$"test-bed/ndtl_param_download.txt"
PERCENT=$"test-bed/percent.txt"
VALUES=$"test-bed/ndtl_values_download.txt"
OUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--param-file-path ${PARAM} \
--percent-file-path ${PERCENT} \
--values-file-path ${VALUES} \
--output-file ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--msf-llg 11111 \
--afacility-llg 222222 \
--afacility-percent 14 \
--ccy INR \
--entity INIDIA \
--as-on-date 13-04-2020 \
--log-level trace \
--diagnostics-flag false 
