#!/usr/bin/env bash

INPUT=$"test-bed/ExchangeRate.txt"
EXRT=$"test-bed/1000ExchangeRate.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ex-rt-file ${EXRT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ccy INR \
--lcy RUP \
--fcy FCY \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
