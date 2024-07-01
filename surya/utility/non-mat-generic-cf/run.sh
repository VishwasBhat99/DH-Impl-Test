#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
INP_META=$"test-bed/miles-input-metadata.json"
OUTPUT=$"test-bed/output"
OUT_META=$"test-bed/miles-output-metadata.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--input-metadata-file ${INP_META} \
--output-metadata-file ${OUT_META} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 12-07-2022 \
--input-date-format dd-mm-yyyy \
--log-level error
