#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/input.xlsx"
OUTPUT_FILE=$"test-bed/output"
SOLTYPE_FILE=$"test-bed/sol-type.txt"

rm test-bed/output_PR??.txt

cargo run --release -- \
--input-file ${INPUT_FILE} \
--input-sheet-name Sheet1 \
--sol-type-file ${SOLTYPE_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-03-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--writer-vec PR01:E,PR02:F,PR03:G,PR04:H,PR05:I,PR06:J,PR07:K,PR08:L,PR09:M,PR10:N,PR11:O,PR12:P,PR13:Q,PR14:R,PR15:S
#--log-level trace \
#--diagnostics-flag false
