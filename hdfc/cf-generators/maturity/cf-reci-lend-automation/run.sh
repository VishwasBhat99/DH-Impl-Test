#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CP_FILE=$"test-bed/cp.txt"
MM=$"test-bed/mm.csv"
OUTPUT_FILE=$"test-bed/output"

cargo run --release -- \
--counter-party-file ${CP_FILE} \
--master-mm-file ${MM} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
