#!/usr/bin/env bash

INPUT=$"test-bed/Forward.xlsx"
OUT=$"test-bed/output"
CCF=$"test-bed/ccf.txt"
RW=$"test-bed/rw.txt"
EXFIELDS_FILE=$"test-bed/extra-fields.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/metadata.json"

cargo run --release -- \
--input-file ${INPUT} \
--input-sheet-name "Sheet1" \
--output-file ${OUT} \
--ccf-file ${CCF} \
--rw-file ${RW} \
--exfields-file ${EXFIELDS_FILE} \
--metadata-file ${METADATA_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-09-2022 \
--log-level trace \
--diagnostics-flag true \
--fields-with-date 10,11
