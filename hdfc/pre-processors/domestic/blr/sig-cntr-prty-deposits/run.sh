#!/usr/bin/env bash

TOP_N_SIG_COUNT=100
SIG_PERC=1.2
INPUT_FILE=$"test-bed/pp-deposits.txt"
LIABILITY_BAL_FILE=$"test-bed/liability-bal.txt"
OUTPUT_FILE=$"test-bed/output/sig-cntrprty-deposits.txt"
LOG_FILE=$"test-bed/output/sig-cntrprty-deposits-log.txt"
DIAGNOSTICS_LOG_FILE=$"test-bed/output/sig-cntrprty-deposits-diag-log.txt"

rm -f ../test-bed/sigcntrprtydep/*

cargo run --release -- \
--top_n_sig_count ${TOP_N_SIG_COUNT} \
--sig_perc ${SIG_PERC} \
--input_file ${INPUT_FILE} \
--liability_bal_file ${LIABILITY_BAL_FILE} \
--output_file ${OUTPUT_FILE} \
--log_file ${LOG_FILE} \
--diag_log_file ${DIAGNOSTICS_LOG_FILE} 
