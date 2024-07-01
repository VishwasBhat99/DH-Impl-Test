#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.txt"
OUTPUT_FILE=$"test-bed/output.txt"
SORTED_OUTPUT=$"test-bed/output_sorted.txt"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  14-02-2022 \
--loan-type EI \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false

#Sorting on ACID and Flow_Start_Date
cat $OUTPUT_FILE | sort -t "|" -k1,1 -k4.7,4.10 -k4.4,4.5 -k4.1,4.2 -s > $SORTED_OUTPUT
