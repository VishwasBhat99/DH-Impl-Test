#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-repo-and-reverse-repo.cf"
RULES_FILE=$"test-bed/rules-repo-and-reverse-repo.txt"
METADATA_FILE=$"test-bed/metadata-repo-and-reverse-repo.json"
OUTPUT=$"test-bed/output/stmp-repo-and-reverse-repo"
LOG_FILE=$"test-bed/output/stmp-log-repo-and-reverse-repo.txt"
DIAGNOSTICS_FILE=$"test-bed/output/stmp-diag-log-repo-and-reverse-repo.txt"

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
