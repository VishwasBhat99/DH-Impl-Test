#!/usr/bin/env bash

INPUT=$"test-bed/STG_SUMM_OPT_REG_MO_OPT_ALL_OS.csv"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2019 \
--log-level trace \
--diagnostics-flag true 
 