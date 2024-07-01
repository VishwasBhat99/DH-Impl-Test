#!/usr/bin/env bash
INPUT_BLOOM=$"test-bed/BLOOMBERG_PRICE_DATA.csv"
EXCHANGE=$"test-bed/CcyExchange.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file-path ${LOG_FILE} \
--output-file ${OUTPUT} \
--input-bloom-file-path ${INPUT_BLOOM} \
--exchange-file-path ${EXCHANGE} \
--log-level trace \
--as-on-date 31-05-2023 \
--face-value "100" \
--order-number "1001" \
--transaction-type "MTN" \
--default-ccy "INR" \
--diagnostics-flag true

