#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
ALM_TRADE_FINANCE_INPUT_FILE=$"test-bed/Alm_trade_finance_input.txt"
TRADE_FINANCE_PP_OUTPUT_FILE=$"test-bed/Trade_Finance_PP_output.txt"


cargo run --release -- \
--alm-trade-finance-input-file ${ALM_TRADE_FINANCE_INPUT_FILE} \
--trade-finance-pp-output-file ${TRADE_FINANCE_PP_OUTPUT_FILE} \
--as-on-date  15-07-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false