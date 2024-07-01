#!/usr/bin/env bash

LOG_FILE=$""
DIAGNOSTICS_FILE=$""

cargo run --release -- \
--input-file "test-bed/extra-fields.txt" \
--output-file "test-bed/final_la.txt" \
--sheet-file "test-bed/Restructured.xlsx" \
--log-file "test-bed/log.txt" \
--diagnostics-log-file "test-bed/diag-log.txt" \
--log-level trace \
--diagnostics-flag false
