#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
SUMMARY_FILE=$"test-bed/summary.txt"
CONFIG_FILE=$"test-bed/config.txt"
OUTPUT_P1=$"test-bed/$1/outputP1.txt"
OUTPUT_P2=$"test-bed/$1/outputP2.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--summary-file ${SUMMARY_FILE} \
--config-file ${CONFIG_FILE} \
--output-file-p1 ${OUTPUT_P1} \
--output-file-p2 ${OUTPUT_P2} \
--country "IND" \
--currency "INR" \
--as-on-date 15-09-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true