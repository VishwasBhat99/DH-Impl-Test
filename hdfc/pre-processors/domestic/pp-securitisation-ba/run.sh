#!/usr/bin/env bash

INPUT=$"test-bed/BA_securitization_102019.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--sheet-name Sheet1 \
--log-level trace \
--llg-code 1004 \
--diagnostics-flag true
