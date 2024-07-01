#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
BGL_CGL_FILE=$"test-bed/bgl_cgl.txt"
MASTER_FILE=$"test-bed/master.xlsx"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--bgl-cgl-file ${BGL_CGL_FILE} \
--master-file ${MASTER_FILE} \
--sheet-name Sheet1 \
--output-file ${OUTPUT_FILE} \
--currency INR \
--country IND \
--branch-code 1 \
--start-mat-date 20,4 \
--int-rate 8.00 \
--as-on-date 28-02-2022 
#--log-level trace \
#--diagnostics-flag true
