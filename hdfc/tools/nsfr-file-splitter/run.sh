#!/usr/bin/env bash

cargo run --release -- \
--input-file "test-bed/input.csv" \
--re-struct-file "test-bed/re-struct.csv" \
--resid-mort-file "test-bed/resid.csv" \
--rw-file "test-bed/rw.csv" \
--inp-del "|" \
--re-struct-field-pos "1" \
--resid-field-pos "30" \
--rw-field-pos "3" \
--acc-no-pos "1" \
--src-codes "BI","HI" \
--re-struct-desc "Y" \
--resid-desc "Claims Secured by ResidEntial Property" \
--rw-desc "0.35" \
--comparator "<=" \
--log-file "test-bed/log.txt" \
--exp-def-flag-pos 17 \
--cap-mkt-exp-pos 19 \
--exp-def-flag-desc "N" \
--cap-mkt-exp-desc "N" \
--diagnostics-log-file "test-bed/diag-log.txt"
--remove-last-char "3"
