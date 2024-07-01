#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-import-loans.cf"
RULES_FILE=$"test-bed/rules-import-loans.txt"
METADATA_FILE=$"test-bed/metadata-import-loans.json"
OUTPUT=$"test-bed/output/stmp-import-loans"
LOG_FILE=$"test-bed/output/stmp-log-import-loans.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-import-loans.txt"

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
