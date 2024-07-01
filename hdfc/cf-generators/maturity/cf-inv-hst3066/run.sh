#!/usr/bin/env bash

INPUT=$"testbed/hst3066.txt"
OUTPUT=$"testbed/hst3066"
LOG_FILE=$"testbed/hst3066-log.txt"
DIAGNOSTICS_FILE=$"testbed/hst3066-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false \
--as-on-date 31-01-2019 \
