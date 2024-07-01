#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
INP_META=$"test-bed/input-meta.json"
OUTPUT=$"test-bed/Output"
OUT_META=$"test-bed/output-meta.json"
REQ_FILE=$"test-bed/req-fields-file.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--input-metadata-file ${INP_META} \
--output-metadata-file ${OUT_META} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-06-2023 \
--input-date-format dd-mm-yyyy \
--required-fields-file ${REQ_FILE}
