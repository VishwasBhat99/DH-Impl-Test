#!/usr/bin/env bash

cargo run --release -- \
--diagnostics-log-file $6 \
--input-file-path $1 \
--log-file $5 \
--metadata-file-path $3 \
--output-file-path $2 \
--required-fields-file-path $4 \
--log-level trace \
--diagnostics-flag false
