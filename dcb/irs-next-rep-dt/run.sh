#!/usr/bin/env bash
INPUT=$"test-bed/input.txt"
REP_MASTER=$"test-bed/rep.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--input-file ${INPUT} \
--rep-file-path ${REP_MASTER} \
--output-file ${OUTPUT_FILE} \
--as-on-date  05-11-2023 \
--log-file ${LOG_FILE} \
--rep-sheet-name "Sheet1" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
