#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-casa.cf"
RULES_FILE=$"test-bed/rules-casa.txt"
METADATA_FILE=$"test-bed/metadata-casa.json"
OUTPUT=$"test-bed/output/stmp-casa"
LOG_FILE=$"test-bed/output/stmp-log-casa.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-casa.txt"

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
