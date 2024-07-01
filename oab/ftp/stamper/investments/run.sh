#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-investments.cf"
RULES_FILE=$"test-bed/rules-investments.txt"
METADATA_FILE=$"test-bed/metadata-investments.json"
OUTPUT=$"test-bed/output/stmp-investments"
LOG_FILE=$"test-bed/output/stmp-log-investments.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-investments.txt"

rm -f test-bed/output/*

cargo run --release -- \
--input-file ${INPUT} \
--rule-file ${RULES_FILE} \
--stamp-field rl1 \
--default-stamp-code 1002 \
--metadata-file ${METADATA_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
