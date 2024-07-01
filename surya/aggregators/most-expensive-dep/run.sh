#!/usr/bin/env bash

LOG_FILE=$""
DIAGNOSTICS_FILE=$""

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
