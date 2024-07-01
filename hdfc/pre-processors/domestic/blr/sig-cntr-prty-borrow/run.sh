#!/usr/bin/env bash

TOP_N_SIG_COUNT=100
SIG_PERC=1.2
INPUT_FILE=$"../ouput/pp-borrow.txt"
TBF=$"../ouput/tot-bal.txt"
OUTPUT_FILE=$"../ouput/output/sig-cntrprty-borrow.txt"
LOG_FILE=$"../ouput/output/sig-cntrprty-borrow-log.txt"
DIAGNOSTICS_LOG_FILE=$"../ouput/output/sig-cntrprty-borrow-diag-log.txt"

cargo run --release -- \
--top-n-sigcount 10 \
--sig-perc 1.0 \
--input-file ${INPUT-FILE} \
--tot-bal-file ${TBF} \
--output-file ${OUTPUT-FILE} \
--log-file ${LOG-FILE} \
--diag-log-file ${DIAGNOSTICS-LOG-FILE} 
