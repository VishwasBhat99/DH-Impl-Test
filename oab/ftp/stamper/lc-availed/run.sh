#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-lc-availed.cf"
RULES_FILE=$"test-bed/rules-lc-availed.txt"
METADATA_FILE=$"test-bed/metadata-lc-availed.json"
OUTPUT=$"test-bed/output/stmp-lc-availed"
LOG_FILE=$"test-bed/output/stmp-log-lc-availed.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-lc-availed.txt"

rm -f test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--rule-file ${RULES_FILE} \
--default-rl1 1002 \
--metadata-file ${METADATA_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
