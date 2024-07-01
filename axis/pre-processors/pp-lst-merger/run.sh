#!/usr/bin/env bash

INPUT=$"test-bed/"
CONFIG=$"test-bed/CONFIG.xlsx"
OUTPUT=$"test-bed/lst_output.txt"
LOG_FILE=$"test-bed/pp_lst_log.txt"
DIAGNOSTICS_FILE=$"test-bed/pp_lst_diag-log.txt"

cargo run -- \
--input-file-path ${INPUT} \
--config-file ${CONFIG} \
--config-sheet-name "Sheet1" \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-08-2022 \
--concat-fields 1,2,3,5,8,9,10,11 
#--log-level trace \
#--diagnostics-flag true
