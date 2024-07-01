#!/usr/bin/env bash

INPUT=$"test-bed/sam.txt"
OUT=$"test-bed/output.txt"
OVR=$"test-bed/Ovedue_rate.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--over-rate-file-path ${OVR} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 21-04-2020 \
--log-level trace \
--diagnostics-flag true \
