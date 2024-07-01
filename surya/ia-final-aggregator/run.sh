#!/usr/bin/env bash

PRINCIPAL=$"test-bed/aggregated_principal_amt.txt"
RATE=$"test-bed/aggregated_rate.txt"
SUMMARY=$"test-bed/aggregated_smry.txt"
OUTPUT=$"test-bed/output/final_aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--principal-file-path ${PRINCIPAL} \
--rate-file-path ${RATE} \
--summary-file-path ${SUMMARY} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--as-on-date 02-02-2020 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} 
#--log-level trace \
#--diagnostics-flag true
