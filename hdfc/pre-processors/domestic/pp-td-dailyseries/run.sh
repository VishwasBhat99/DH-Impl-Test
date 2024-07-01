#!/usr/bin/env bash

LOG_FILE=$"test-bed/log_file.txt"
DIAGNOSTICS_FILE=$"test-bed/diagnostic_log_file.txt"
INPUT_FILE=$"test-bed/input.txt"
RULE_CURR_FILE=$"test-bed/input_curr.txt"
OUTPUT_FILE=$"test-bed/output.txt"


cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--rule-file ${RULE_CURR_FILE} \
--base-currency "RUP" 
