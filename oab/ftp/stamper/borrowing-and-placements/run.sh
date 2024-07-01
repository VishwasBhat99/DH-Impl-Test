#!/usr/bin/env bash

INPUT=$"test-bed/stmp-temp.cf"
RULES_FILE=$"test-bed/aorl-rules-borrowing.txt"
METADATA_FILE=$"test-bed/metadata-borrowing-and-placements.json"
OUTPUT=$"test-bed/stmp-temp-aorl"
LOG_FILE=$"test-bed/output/stmp-log-placements.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-placements.txt"

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
