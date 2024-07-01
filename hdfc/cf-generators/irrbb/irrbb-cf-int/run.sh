#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2022 \
--input-metadata-file "test-bed/out-meta.json" \
--required-fields-file "test-bed/req.json" \
#--is-cf-lvl-data true \
#--log-level trace \
#-diagnostics-flag true
