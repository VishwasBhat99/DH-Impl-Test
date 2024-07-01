#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.txt"
NET=$"test-bed/net.txt"
LLG=$"test-bed/llg.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--inflow-outflow-rule-file ${LLG} \
--currency INR \
--net-rule-file ${NET} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
