#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/bor-log.txt"
DIAGNOSTICS_FILE=$"test-bed/bor-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--entity INDIA_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2019 \
--log-level trace \
--diagnostics-flag true 
