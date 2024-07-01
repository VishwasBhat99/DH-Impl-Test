#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
CF_FILE=$"test-bed/cashflows.txt"
GL_MASTER=$"test-bed/gl_master.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MAPPING_MASTER=$"test-bed/mapping_master.txt"

cargo run -- \
--input-file-path ${INPUT} \
--cashflow-file-path ${CF_FILE} \
--investments-gl-master ${GL_MASTER} \
--mapping-master ${MAPPING_MASTER} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-06-2023
