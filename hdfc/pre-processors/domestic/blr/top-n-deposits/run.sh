#!/usr/bin/env bash

TOP_N_CUST_COUNT=150
INPUT_FILE=$"test-bed/pp-deposits.txt"
OUTPUT_FILE=$"test-bed/output/top-n-deposits.txt"
LOG_FILE=$"test-bed/output/top-n-deposits-log.txt"
DIAGNOSTICS_LOG_FILE=$"test-bed/output/top-n-deposits-diag-log.txt"

rm -f ../test-bed/topndep/*

cargo run --release -- \
--input_file ${INPUT_FILE} \
--top_n_cust_count ${TOP_N_CUST_COUNT} \
--output_file ${OUTPUT_FILE} \
--log_file ${LOG_FILE} \
--diag_log_file ${DIAGNOSTICS_LOG_FILE} 
