#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
ASSET_CLASS=$"test-bed/lcr_classification_master_basel.xlsx"
LCR_CAT=$"test-bed/lcr_classification_master.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--asset-class-path ${ASSET_CLASS} \
--lcr-cat-path ${LCR_CAT} \
--v-src-system-ids "FW,VP,HLTD" \
--asset-sheet-name "Sheet1" \
--lcr-sheet-name "OThers" \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level debug 
# --diagnostics-flag true
