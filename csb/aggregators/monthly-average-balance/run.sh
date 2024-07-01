#!/usr/bin/env bash

INPUT=$"test-bed/input/30062021/output_repo.txt"
OUTPUT=$"test-bed/output/out.txt"
CLOSE=$"test-bed/output/close.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--close-accounts-file ${CLOSE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-06-2021 \
--date-format DDMMYYYY \
