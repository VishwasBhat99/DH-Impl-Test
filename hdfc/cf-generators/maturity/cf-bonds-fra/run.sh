#!/usr/bin/env bash

INPUT=$"test-bed/output.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
    
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--entity INDIA_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2021 \
--log-level trace \
--is_nd false \
--diagnostics-flag true 
