#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
REPAY=$"test-bed/res.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--repayment-struct-file ${REPAY} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--log-level trace \
--as-on-date 30-09-2020 \
--diagnostics-flag true 
