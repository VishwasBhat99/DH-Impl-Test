#!/usr/bin/env bash

INPUT=$"test-bed/input/27012019/FinnoneLoans.txt"
OUTPUT=$"test-bed/output/out.txt"
OPEN=$"test-bed/output/open.txt"
CLOSE=$"test-bed/output/close.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--open-accounts-file ${OPEN} \
--close-accounts-file ${CLOSE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019 \
--date-format DDMMYYYY \
