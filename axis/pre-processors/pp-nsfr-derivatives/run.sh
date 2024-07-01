#!/usr/bin/env bash

INPUT_FILE=$"test-bed/nsfr/NSFR_Derivative.txt"
OUTPUT=$"test-bed/nsfr/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXCHANGE_RATE=$"test-bed/nsfr/1000ExchangeRate.txt"
COMMON_CODE=$"test-bed/nsfr/common_code.txt"

cargo run -- \
--input-file-path ${INPUT_FILE} \
--exchange-rate-file ${EXCHANGE_RATE} \
--output-file ${OUTPUT} \
--base-currency INR \
--log-file ${LOG_FILE} \
--bkt-schema-file-path ${COMMON_CODE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 22-02-2024 \
--country "INDIA"
