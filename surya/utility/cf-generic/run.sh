#!/usr/bin/env bash

INPUT_FILE=$"test-bed/pp-out-inv-dnbs_Sheet1.txt"
INP_META=$"test-bed/inv_pp-md.json"
OUTPUT=$"test-bed/out"
OUT_META=$"test-bed/inv_cf-md.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--input-metadata-file ${INP_META} \
--output-metadata-file ${OUT_META} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 01-07-2022 \
--input-date-format dd-mm-yyyy \
--cf-fields-col 0.0,0.0,mat_date
