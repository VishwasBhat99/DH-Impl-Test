#!/usr/bin/env bash

INPUT=$""
OUTPUT=$""
LOG_FILE=$""
DIAGNOSTICS_FILE=$""

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--sheet-name "Sheet1" \
--as-on-date 01-01-2019 
