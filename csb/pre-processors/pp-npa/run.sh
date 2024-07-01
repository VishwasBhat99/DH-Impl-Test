#!/usr/bin/env bash

NPA=$"test-bed/npa.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT=$"test-bed/output.txt"

cargo run --release -- \
--npa-consolidated ${NPA} \
--output-file-path ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
