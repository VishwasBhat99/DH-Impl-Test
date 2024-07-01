#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/non-ret-output-LCY.txt"
FINAL_INPUT_FILE=$"test-bed/td-nr-final.txt"
OUTPUT_FILE=$"test-bed/output"
LLG_MAPPER=$"test-bed/llg_mapper.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--final-input-file ${FINAL_INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-03-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--allocation-order DESC \
--llg-mapper-file ${LLG_MAPPER} \
--country IND \
#--log-level trace \
#--diagnostics-flag false
