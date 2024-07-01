#!/usr/bin/env bash

INP=$"test-bed/lcbg.xlsx"
REF=$"test-bed/input.xlsx"
OUT=$"test-bed/out"
SLAB=$"test-bed/slab.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--input-file-path ${INP} \
--ref-file-path ${REF} \
--slab-file-path ${SLAB} \
--log-file-path ${LOG_FILE} \
--output-file-path ${OUT} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-19 \
--log-level trace \
--input-sheet-name "Sheet 1" \
--master-sheet-name "Sheet 1" 
#--is-perf-diagnostics-enabled false 

