#!/usr/bin/env bash

INPUT=$"test-bed/pp-borrow-lend.txt"
OUTPUT=$"test-bed/output/cf-gen-output"
LOG_FILE=$"test-bed/bor-lend-log.txt"
DIAGNOSTICS_FILE=$"test-bed/bor-lend-diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--convention ACTbyACT \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2019 \
--log-level trace \
--diagnostics-flag true 
