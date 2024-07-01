#!/usr/bin/env bash

INPUT=$"test-bed/input/31-01-2019/ftp-ca.txt"
OUTPUT=$"test-bed/output/consol-ftp-ca.txt"
LOG_FILE=$"test-bed/output/consol-ftp-log-ca.txt"
DIAGNOSTICS_FILE=$"test-bed/output/consol-ftp-diag-log-ca.txt"

rm test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-01-2019 \
--date-format DD-MM-YYYY
