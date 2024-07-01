#!/usr/bin/env bash

INPUT=$"test-bed/sample.txt"
OUTPUT=$"test-bed/TDCFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

//read as_on_dt < ../../../common_resources.txt

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2019 \
--day-convention ACT/365
#--log-level trace \
#-diagnostics-flag true
