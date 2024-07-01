#!/usr/bin/env bash
INPUT="test-bed/input.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUT="test-bed/output.txt"
MASTER=$"test-bed/master.txt"

cargo run -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file-path ${INPUT} \
--log-file ${LOG_FILE} \
--master-file-path ${MASTER} \
--output-file-path ${OUT} \
--as-on-date 30-06-2023
