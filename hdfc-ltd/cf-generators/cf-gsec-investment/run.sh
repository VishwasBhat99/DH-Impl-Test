#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/gsec-output"
LOG_FILE=$"test-bed/gsec-log.txt"
DIAGNOSTICS_FILE=$"test-bed/gsec-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--convention ACTby365 \
--amt-type market_val \
--log-level trace \
--as-on-date 21-01-2021 \
--diagnostics-flag true 
