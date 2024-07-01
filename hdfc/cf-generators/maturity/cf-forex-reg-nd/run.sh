#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output"
REF=$"test-bed/ref.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
    
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--ref-file ${REF} \
--entity INDIA_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2015 \
--log-level trace \
--diagnostics-flag true 
