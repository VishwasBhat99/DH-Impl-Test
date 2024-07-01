#!/usr/bin/env bash

cargo run --release -- \
--diagnostics-log-file "test-bed/fiag_log.txt" \
--log-file "test-bed/log.txt" \
--log-level trace \
--diagnostics-flag false \
--day "FRI" \
--as-on-date 14-05-2021
