#!/usr/bin/env bash

INPUT_FILE=$"test-bed/CC_LOAN_AMORT_EXT"
MASTER=$"test-bed/master.csv"
OUTPUT=$"test-bed/Output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--master-file ${MASTER} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true 
