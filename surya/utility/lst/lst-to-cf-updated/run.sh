#!/usr/bin/env bash

INPUT=$"test-bed/0006billsothers.lst"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REF_FILE=$"test-bed/Book.xlsx"
REQ_FIELDS_FILE=$"test-bed/req.json"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--ref-file ${REF_FILE} \
--req-fields-file ${REQ_FIELDS_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 06-08-2019
