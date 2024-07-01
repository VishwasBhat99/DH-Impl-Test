#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output"
REF_FILE=$"test-bed/ref.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log.txt"

cargo run  -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--ref-file-path ${REF_FILE} \
--sheet-name Sheet1 \
--entity INDIA_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2019 \
--log-level trace \
--diagnostics-flag true 
