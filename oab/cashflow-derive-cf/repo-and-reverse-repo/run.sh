#!/usr/bin/env bash

INPUT=$"test-bed/pp-out-repo-and-reverse-repo.txt"
OUTPUT=$"test-bed/cf-out-repo-and-reverse-repo"
LOG_FILE=$"test-bed/cf-log-repo-and-reverse-repo.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-diag-log-repo-and-reverse-repo.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 
