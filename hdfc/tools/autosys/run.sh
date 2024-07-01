#!/usr/bin/env bash

cargo run --release -- \
--connection-string "https://127.0.0.1" \
--as-on-date "2023-10-10" \
--batch-id "1" \
--stream-ids "9999" \
--wait-time-in-sec 60 \
--max-retry 3 \
--accept-invalid-certs true 
# Wont Validate the SSL/TLS certificates
# --accept-invalid-certs false \
# Will validate the SSL/TLS certificates