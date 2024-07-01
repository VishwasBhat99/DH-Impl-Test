#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/input.txt"
SALARY_PENSION_DATA=$"test-bed/salary_data.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run -- \
--input-file-path ${INPUT_FILE} \
--salary-pension-data ${SALARY_PENSION_DATA} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
--country-code IND \
--diagnostics-flag true
