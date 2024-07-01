#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
CONFIG=$"test-bed/ABconfig.json"
LCR_MASTER_BASEL=$"test-bed/lcr_classification_master_basel.xlsx"
LINE_CODE=$"test-bed/line_template_undrawn.xlsx"
ODFD=$"test-bed/odfc.xlsx"
LCR_CLASS=$"test-bed/lcr_classification_master.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--config-file ${CONFIG} \
--lcr-master-basel-path ${LCR_MASTER_BASEL} \
--line-temp-undrawn ${LINE_CODE} \
--lcr-master-file ${LCR_CLASS} \
--odfd-path ${ODFD} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--derived-flag "LNM" \
--as-on-date 31-01-2019 \
--log-level debug 
# --diagnostics-flag true
