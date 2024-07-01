#!/usr/bin/env bash

INPUT=$"test-bed/folders.json"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--files-path ${INPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--months 5 \
--log-level trace \
--diagnostics-flag true
