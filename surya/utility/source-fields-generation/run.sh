#!/usr/bin/env bash

OUTPUT=$"test-bed/output.txt"
CONFIG=$"test-bed/config.json"
LOG=$"test-bed/log.txt"
DIAG_LOG=$"test-bed/diag-log.txt"

cargo run --release -- \
--diagnostics-log-file $DIAG_LOG \
--log-file $LOG \
--as-on-date 31-01-2021 \
--config-file-path $CONFIG \
--output-file-path $OUTPUT \
#--log-level trace \
#--diagnostics-flag false
