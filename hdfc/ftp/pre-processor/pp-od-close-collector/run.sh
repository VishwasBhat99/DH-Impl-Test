#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output"
AMB=$"test-bed/amb.txt"
CORE=$"test-bed/core-master.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--amb-file ${AMB} \
--core-master ${CORE} \
--core-master-sheet-name Sheet1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--day-convention ACT/365
#--log-level trace \
#-diagnostics-flag true
