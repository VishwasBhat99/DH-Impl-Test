#!/usr/bin/env bash

RET_INPUT=$"test-bed/ret-input.txt"
NON_RET_INPUT=$"test-bed/non-ret-input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ret-input-file ${RET_INPUT} \
--non-ret-input-file ${NON_RET_INPUT} \
--as-on-date 31-08-2021 
#--log-level trace \
#--diagnostics-flag true
