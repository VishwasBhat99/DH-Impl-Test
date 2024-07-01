#!/usr/bin/env bash

INPUT=$"test-bed/ledbal.csv"
OUT=$"test-bed/output.txt"
CON=$"test-bed/concat.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/MappingMaster_v3.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--concat-file ${CON} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--is-header true \
--log-level trace \
--diagnostics-flag true \
