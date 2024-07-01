#!/usr/bin/env bash

LIEN_INPUT=$"test-bed/lien-input.txt"
TD_INPUT=$"test-bed/td-input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--lien-input-file ${BILLS_INPUT} \
--td-input-file ${OD_INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 25-03-2023 \
--log-level trace \
#--diagnostics-flag true
