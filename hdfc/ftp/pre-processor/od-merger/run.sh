#!/usr/bin/env bash

CORE=$"test-bed/core_1.txt"
CORE_ADD=$"test-bed/core.txt"
NONCORE=$"test-bed/non_core_2.txt"
NONCORE_ADD=$"test-bed/non-core.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--core-file ${CORE} \
--non-core-file ${NONCORE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--core-add-file ${CORE_ADD} \
--non-core-add-file ${NONCORE_ADD} \
#--log-level trace \
#-diagnostics-flag true
