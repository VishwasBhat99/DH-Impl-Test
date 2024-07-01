#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-undrawn-commitments.cf"
RULES_FILE=$"test-bed/rules-undrawn-commitments.txt"
METADATA_FILE=$"test-bed/metadata-undrawn-commitments.json"
OUTPUT=$"test-bed/output/stmp-undrawn-commitments"
LOG_FILE=$"test-bed/output/stmp-log-undrawn-commitments.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-undrawn-commitments.txt"

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
