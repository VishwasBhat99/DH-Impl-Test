#!/usr/bin/env bash
INPUT_FILE="test-bed/Files.json"
LOG_FILE="test-bed/Testlog.txt"
OUTPUT="test-bed/Output_File"
DIAGNOSTICS_FILE="test-bed/TestDiaglog.txt"

cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
