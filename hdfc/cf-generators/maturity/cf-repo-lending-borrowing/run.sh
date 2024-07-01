#!/usr/bin/env bash

INPUT=$"../repo_lending_borrowing_pre/testbed/hst3015.txt"
OUTPUT=$"testbed/hst3015"
LOG_FILE=$"testbed/hst3015-log.txt"
DIAGNOSTICS_FILE=$"testbed/hst3015-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false \
--as-on-date 31-01-2019
