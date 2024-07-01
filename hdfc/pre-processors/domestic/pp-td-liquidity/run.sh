#!/usr/bin/env bash

LOG_FILE=$"/home/atul/Documents/SuperDB-Batch/hdfc/pre-processors/domestic/pp-td-liquidity/test-bed/log_file.txt"
DIAGNOSTICS_FILE=$"/home/atul/Documents/SuperDB-Batch/hdfc/pre-processors/domestic/pp-td-liquidity/test-bed/diagnostic_log_file.txt"
INPUT_FILE=$"/home/atul/Documents/SuperDB-Batch/hdfc/pre-processors/domestic/pp-td-liquidity/test-bed/input.txt"
RULE_CURR_FILE=$"/home/atul/Documents/SuperDB-Batch/hdfc/pre-processors/domestic/pp-td-liquidity/test-bed/input_curr.txt"
OUTPUT_FILE=$"/home/atul/Documents/SuperDB-Batch/hdfc/pre-processors/domestic/pp-td-liquidity/test-bed/output.txt"


cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--rule-file ${RULE_CURR_FILE} \
--base-currency "RUP" 
