#!/usr/bin/env bash
CONFIG=$"test-bed/ABConfig.json"
RTH=$"test-bed/31102023/curr_rth.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--balm-rc-rth-file ${RTH} \
--config-file-path ${CONFIG} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-10-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
