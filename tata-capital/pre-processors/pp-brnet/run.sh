#!/usr/bin/env bash

INPUT_FILE=$"test-bed/ALM_SURYA_BRNET_LoanDumpDetail_31102023.txt"
TCFSL_FILE=$"test-bed/TCFSL.xlsx"
SCHEDULE_DUMP_FILE=$"test-bed/ALM_SURYA_BRNET_SheduleDumpDetail_31102023.txt"
WRITEOFf_MERGED=$"test-bed/WRITEOFF_MERGED.txt"
OUTPUT_FILE=$"test-bed/td_output.txt"
BRNET_FILE=$"test-bed/ALM_SURYA_BRNET.txt".
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--sc-dump-file ${SCHEDULE_DUMP_FILE} \
--brnet-file ${BRNET_FILE} \
--tcfsl-sheet-name "Stage III" \
--tcfsl-file  ${TCFSL_FILE} \
--writeoff-merged-file ${WRITEOFF_MERGED} \
--output-file ${OUTPUT_FILE} \
--input-date-format "yyyy-mm-dd" \
--as-on-date 31-10-2023
# --log-level trace \
# --diagnostics-flag true
