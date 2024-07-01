#!/usr/bin/env bash
INPUT_FILE=$"test-bed/input.txt"
COUPON_MASTER=$"test-bed/Master_Yield.xlsx"
EXCHANGE=$"test-bed/exc_rate.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"
EXCH=$"test-bed/exch.txt"

cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--coupon-master-file ${COUPON_MASTER} \
--exchange-rate-file ${EXCHANGE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  30-04-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--coupon-sheet-name "Sheet1" \
--exchange-rate-out ${EXCH} \
--consolidated-ccy "USD" \
--local-ccy "USL" \

#--log-level trace \
#--diagnostics-flag false
