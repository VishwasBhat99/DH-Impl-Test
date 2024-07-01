#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
LCR_CAT=$"test-bed/lcr_cat_file.xlsx"
CD_OD=$"test-bed/cd_od.xlsx"
FUNDED=$"test-bed/funded_file.xlsx"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
LCR_BAS=$"test-bed/lcr_master_basel.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--lcr-cat-path ${LCR_CAT} \
--cd-od-path ${CD_OD} \
--funded-path ${FUNDED} \
--cd-od-sheet-name "Sheet1" \
--funded-sheet-name "Sheet1" \
--lcr-cat-sheet "Sheet1" \
--lcr-master-basel-path ${LCR_BAS} \
--lcr-master-sheet-name "Sheet1" \
--output-file ${OUTPUT} \
--cust-ref-code 329 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level debug 
# --diagnostics-flag true
