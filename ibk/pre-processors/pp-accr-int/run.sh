#!/usr/bin/env bash

INPUT_FILE=$"test-bed/Input.txt"
HOLIDAY=$"test-bed/Holiday.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--holiday-file ${HOLIDAY} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-08-2022 \
--input-file-delimiter "|" \
--holiday-file-delimiter "|" \
--input-date-format "yyyy.mm.dd" \
--log-level trace \
--diagnostics-flag true
