#!/usr/bin/env bash

INPUT=$"test-bed/aggregated_smry.txt"
OUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--src-id "Term Deposits" \
--is-src-tb "Y" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
