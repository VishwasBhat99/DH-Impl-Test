#!/usr/bin/env bash

OUT="test-bed/scp.sh"

cargo run -- \
    --command "scp" \
    --executor-id 4 \
    --output-file-path $OUT \
    --scenario-file-path "test-bed/ftp/" \
    --target-server "dbuser@10.226.202.139" \
    --log-file-path "test-bed/log.txt" \
    --diagnostics-file-path "test-bed/diag-log.txt" \
    --log-level trace \
    --diagnostics-flag true

awk '!seen[$0]++' $OUT >${OUT}_temp
mv ${OUT}_temp $OUT
