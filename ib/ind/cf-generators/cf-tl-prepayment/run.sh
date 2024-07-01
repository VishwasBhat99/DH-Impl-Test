#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
MAPPING_MASTER=$"test-bed/mapping-master.xlsx"
BKT_DEF=$"test-bed/bkt-def.txt"
OUTPUT=$"test-bed/cf-out-td-prepayment"
LOG_FILE=$"test-bed/cf-out-td-prepayment-log.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-out-td-prepayment-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--mapping-master-file ${MAPPING_MASTER} \
--bkt-def-file ${BKT_DEF} \
--mapping-master-sheet-name "Sheet1" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false \
--as-on-date 31-08-2022 
