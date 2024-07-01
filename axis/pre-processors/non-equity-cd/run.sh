#!/usr/bin/env bash
INPUT_OUTSTD=$"test-bed/CDOUTSTANDING_BLR5.csv"
INPUT_TRADE=$"test-bed/CDTraded_BLR5_1.csv"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file-path ${LOG_FILE} \
--output-file ${OUTPUT} \
--input-outstanding-file-path ${INPUT_OUTSTD} \
--input-trade-file-path ${INPUT_TRADE} \
--log-level trace \
--as-on-date 31-05-2023 \
--face-value "100" \
--diagnostics-flag true

