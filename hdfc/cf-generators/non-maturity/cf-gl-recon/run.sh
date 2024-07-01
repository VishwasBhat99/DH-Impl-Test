#!/usr/bin/env bash

INPUT_FILE=$"test-bed/sample.csv"
REF1=$"test-bed/Master_LLG_16022019.xlsx"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--alm-master-file ${REF1} \
--alm-master-sheet-name Master \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level trace \
--diagnostics-flag true
