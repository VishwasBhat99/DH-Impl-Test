#!/usr/bin/env bash

INPUT=$"test-bed/org.csv"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REQ_FIELDS=$"test-bed/req-fields.json"
    
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--sheet-name Sheet1 \
--req-field-file ${REQ_FIELDS} \
--entity INDIA_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2015 \
--log-level trace \
--diagnostics-flag true 
