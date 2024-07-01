#!/usr/bin/env bash

INPUT=$"test-bed/31032024/pp-out-cc-emi.txt"
OUTPUT=$"test-bed/output.txt"
CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--output-file-path ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-03-2024 \
--config-file ${CONFIG} \
--log-level trace \
--diagnostics-flag false
