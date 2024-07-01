#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
OP_FILE_PRIN=$"test-bed/op_prin"
OP_FILE_OD=$"test-bed/op_od"
TENOR=$"test-bed/tenor.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--calculate-int-rt-from-ason N \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INP_FILE} \
--log-file ${LOG_FILE} \
--output-file-principal ${OP_FILE_PRIN} 
