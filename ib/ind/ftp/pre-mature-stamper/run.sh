#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
MASTER=$"test-bed/master.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--input-stamper-file ${INPUT} \
--master-stamper-file ${MASTER} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2022 \
