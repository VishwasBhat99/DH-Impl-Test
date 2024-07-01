#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/DNBS01-Annx-2and3-SHPandBOD.xlsx"
OUTPUT_FILE=$"test-bed/output"

cargo run --release -- \
--input-file ${MASTER_FILE} \
--skip-rows 1,2,3,30,31 \
--sheet-name "Input" \
--output-file ${OUTPUT_FILE} \
--append-required true \
--as-on-date  31-01-2021 \
--fields-with-date 0 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
