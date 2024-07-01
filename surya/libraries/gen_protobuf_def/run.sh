#!/usr/bin/env bash

LOG_FILE=$"/Surya/Automated_proto/test-bed/log.txt"
DIAGNOSTICS_FILE=$"/Surya/Automated_proto/test-bed/diag-log.txt"
INPUT_FILE=$"/home/pavan/Downloads/csb_undrawn.json"
OUTPUT_FILE=$"simple.proto"
cargo run --release -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
