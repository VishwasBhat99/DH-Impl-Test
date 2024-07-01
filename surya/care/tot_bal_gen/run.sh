#!/usr/bin/env bash

INPUT_CONFIG="test-bed/config.json"
OUTPUT="test-bed/tot-bal"
EXP_BASE="test-bed/exp.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--input-config ${INPUT_CONFIG} \
--output-path ${OUTPUT} \
--exp-base-file ${EXP_BASE} \
--exp-base-file-sheet-name "Sheet1" \
--is-limit-required false \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
