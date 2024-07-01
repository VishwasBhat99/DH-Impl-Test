#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-loan-non-capitalized.cf"
RULES_FILE=$"test-bed/rules-loan-non-capitalized.txt"
METADATA_FILE=$"test-bed/metadata-loan-non-capitalized.json"
OUTPUT=$"test-bed/output/stmp-loan-non-capitalized"
LOG_FILE=$"test-bed/output/stmp-log-loan-non-capitalized.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-loan-non-capitalized.txt"

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
