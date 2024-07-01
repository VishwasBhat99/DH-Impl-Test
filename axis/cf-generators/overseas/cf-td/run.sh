#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
OP_FILE=$"test-bed/op_prin"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--calculate-int-rt-from-ason N \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INP_FILE} \
--log-file ${LOG_FILE} \
--output-file-path ${OP_FILE} \
--key-field "foracid" \
--round-off-ex-rt 7