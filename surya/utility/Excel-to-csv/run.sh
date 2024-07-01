#!/usr/bin/env bash

INPUT_FILE=$"test-bed/pcode18.xlsx"
OUTPUT_PATH=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-path ${OUTPUT_PATH} \
--output-file-names "master-loans.csv,cashflow-loans.csv" \
--sheet-names "outstanding,future cash flows" \
--skip-header "false|false" \
--csv-seperator  "," \
--fields-with-date "1,7|1,9" \
--header-rows "1|1" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date  31-01-2022
#--log-level trace \
#--diagnostics-flag false
