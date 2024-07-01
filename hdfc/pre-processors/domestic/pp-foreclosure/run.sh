#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
--llg-code 1007
