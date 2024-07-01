#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/master.txt"
CF_FILE=$"test-bed/cf.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-master-file ${MASTER_FILE} \
--input-cf-file ${CF_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
