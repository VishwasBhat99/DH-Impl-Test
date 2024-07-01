#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
ALM_OPICS_SECURITIES_INPUT_FILE=$"test-bed/Alm_opics_securities_input.txt"
SECURITIES_PP_OUTPUT_FILE=$"test-bed/Securities_PP_output.txt"


cargo run --release -- \
--alm-opics-securities-input-file ${ALM_OPICS_SECURITIES_INPUT_FILE} \
--securities-pp-output-file ${SECURITIES_PP_OUTPUT_FILE} \
--as-on-date  13-07-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false