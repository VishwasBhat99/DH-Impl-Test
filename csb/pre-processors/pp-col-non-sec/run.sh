#!/usr/bin/env bash

INP=$"test-bed/nonsec.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT=$"test-bed/output.txt"

cargo run --release -- \
--input-file-path ${INP} \
--output-file-path ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--elg-col-type-cd "001,022" \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
