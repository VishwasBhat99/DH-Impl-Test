#!/usr/bin/env bash

INPUT=$"../etreasury_borr_hmm3025_pre/testbed/hmm3025.txt"
OUTPUT=$"testbed/hmm3025"
LOG_FILE=$"testbed/hmm3025-log.txt"
DIAGNOSTICS_FILE=$"testbed/hmm3025-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false \
--as-on-date 31-01-2019 \
