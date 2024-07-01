#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
ALM_BORROWING_INPUT_FILE=$"test-bed/ALM_Borrowing_input.txt"
BORROWING_PP_OUTPUT_FILE=$"test-bed/Borrowing_PP_output.txt"


cargo run --release -- \
--alm-borrowing-file ${ALM_BORROWING_INPUT_FILE} \
--borrowing-pp-file ${BORROWING_PP_OUTPUT_FILE} \
--as-on-date  11-07-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false 