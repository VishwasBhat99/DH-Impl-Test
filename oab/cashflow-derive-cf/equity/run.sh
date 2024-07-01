#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-equity.txt"
DIST_RULE=$"test-bed/dist-rules-casa.txt"
OUTPUT=$"test-bed/cf-out-equity"
LOG_FILE=$"test-bed/cf-log-equity.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-equity.txt"

cargo run --release -- \
--input-file ${INPUT} \
--distribution-rule-file ${DIST_RULE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
