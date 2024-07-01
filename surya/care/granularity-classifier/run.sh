#!/usr/bin/env bash

CONFIG_FILE=$"test-bed/config.json"
OUTPUT=$"test-bed/output.txt"
GRAN_FILE=$"test-bed/gran-wt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXRT_RATE_FILE=$"test-bed/1000ExchangeRate.txt"

cargo run --release -- \
--config-file-path ${CONFIG_FILE} \
--output-file-path ${OUTPUT} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--exchange-rate-file ${EXRT_RATE_FILE} \
--as-on-date 22-02-2022 \
--base-currency INR \
--log-level trace \
--diagnostics-flag true \
--granularity-weight-file ${GRAN_FILE} \
--is-granularity-perc false
