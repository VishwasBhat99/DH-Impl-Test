#!/usr/bin/env bash

OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
CONFIG_FILE=$"test-bed/config.json"

cargo run --release -- \
--report-id ASW \
--home-currency INR \
--display-currency INR \
--consol-currency RUP \
--config-file $CONFIG_FILE \
--output-file $OUTPUT \
--log-file $LOG_FILE \
--diagnostics-log-file $DIAGNOSTICS_FILE \
--as-on-date 30-06-2023 \
--log-level trace \
--diagnostics-flag true
