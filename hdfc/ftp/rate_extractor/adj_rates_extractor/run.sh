#!/usr/bin/env bash

INPUT=$"input/Adj_rates.txt"
OUTPUT=$"output/adj.txt"
LOG_FILE=$"output/logs/adj-rates-extract-log.txt"
DIAGNOSTICS_FILE=$"output/logs/adj-rates-extract-diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
 
