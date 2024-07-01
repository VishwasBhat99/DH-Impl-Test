#!/usr/bin/env bash

INPUT_FILE=$"test-bed/inp.txt"
OUTPUT_FILE=$"test-bed/output.txt"
MASTER_FILE=$"test-bed/master.xlsx"
BGL_CGL=$"test-bed/bgl_cgl.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--bgl-cgl-file ${BGL_CGL} \
--output-file ${OUTPUT_FILE} \
--sheet-name "Sheet1" \
--as-on-date 31-01-2022 \
--currency "INR" \
#--log-level trace \
#--diagnostics-flag true
