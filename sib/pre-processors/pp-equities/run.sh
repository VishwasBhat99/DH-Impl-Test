#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
GL_FILE=$"test-bed/investments_gl.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file-path ${INPUT} \
--investments-gl-file-path ${GL_FILE} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-06-2023
