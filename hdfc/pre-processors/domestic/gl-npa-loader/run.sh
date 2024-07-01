#!/usr/bin/env bash
NPA_FILE=$"test-bed/gl_npa.txt"
RTH=$"test-bed/31102023/curr_rth.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--npa-file-path ${NPA_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-10-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false