#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-gl-balance.cf"
RULES_FILE=$"test-bed/rules-gl-balance.txt"
METADATA_FILE=$"test-bed/metadata-gl-balance.json"
OUTPUT=$"test-bed/output/stmp-gl-balance"
LOG_FILE=$"test-bed/output/stmp-log-gl-balance.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-gl-balance.txt"

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
