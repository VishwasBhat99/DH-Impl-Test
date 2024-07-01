cargo run --release -- \
--input-file-path "test-bed/input.txt" \
--output-file "test-bed/op.txt" \
--log-file "test-bed/log.txt" \
--diagnostics-log-file "test-bed/diag.txt" \
--log-level "info" \
--diagnostics-flag "false" \
--cust-id-column 8 \
--as-on-date "30-08-2023"