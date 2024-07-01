#!/usr/bin/env bash

INPUT_FILE=$"test-bed/"
CASHFLOW_FILE=$"test-bed/cashflow.xlsx"
MAPPING_MASTER=$"test-bed/master.xlsx"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--mis-input-file "mis-output.txt" \
--input-file ${INPUT_FILE} \
--input-cf-file ${CASHFLOW_FILE} \
--mapping-master-file ${MAPPING_MASTER} \
--path-sep '/' \
--cashflow-sheet Sheet1 \
--master-sheet Sheet1 \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 02-08-2022 \
--log-level debug \
--diagnostics-flag true
