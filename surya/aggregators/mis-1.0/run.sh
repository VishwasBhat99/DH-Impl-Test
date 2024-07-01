#!/usr/bin/env bash

OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
CONFIG_FILE=$"test-bed/topnconfig.json"

cargo run --release -- \
--country-code KUWAIT \
--home-currency KWD \
--config-file $CONFIG_FILE \
--output-file $OUTPUT \
--log-file $LOG_FILE \
--diagnostics-log-file $DIAGNOSTICS_FILE \
--as-on-date 31-01-2023 \
--write-aggr-vals true \
--log-level trace \
--diagnostics-flag true
