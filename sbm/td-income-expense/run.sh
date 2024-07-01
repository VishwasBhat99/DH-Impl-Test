#!/usr/bin/env bash

CONFIG=$"test-bed/config.json"
LOG=$"test-bed/log.txt"
DIAG=$"test-bed/diag.txt"

cargo run --release -- \
--as-on-date $1 \
--config-file $CONFIG \
--diagnostics-log-file $DIAG \
--log-file $LOG \
--log-level none
