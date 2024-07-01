#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/encum-output"
LOG_FILE=$"test-bed/encum-log.txt"
DIAGNOSTICS_FILE=$"test-bed/encum-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--as-on-date 28-02-2021 \
--diagnostics-flag true 
