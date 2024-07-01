#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
RES_FILE=$"test-bed/res-1.txt"
OP_FILE=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INP_FILE} \
--restructure-file-path ${RES_FILE} \
--output-file-path ${OP_FILE} \
--as-on-date "15-06-2022" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
