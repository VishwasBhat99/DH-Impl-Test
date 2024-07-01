#!/usr/bin/env bash

INPUT_NEW=$"test-bed/new-file-2.txt"
INPUT_EX_GL=$"test-bed/exclude_gl.txt"
NPA_INPUT=$"test-bed/npa.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/ubs-amb-pp-loans-log.txt"
DIAGNOSTICS_FILE=$"test-bed/ubs-amb-pp-loans-diag-log.txt"

cargo run -- \
--input-file ${INPUT_NEW} \
--gl-file ${INPUT_EX_GL} \
--npa-file ${NPA_INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \