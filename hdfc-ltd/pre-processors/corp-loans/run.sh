#!/usr/bin/env bash

INPUT_MASTER="test-bed/finncorp-loans-input-master.txt"
INPUT_CASHFLOW="test-bed/finncorp-loans-cf-input.txt"
INPUT_LEDGER="test-bed/GL-Ledger.txt"
OUTPUT="test-bed/finncorp-pp-out.txt"
LOG_FILE="test-bed/cf-log-corp-loans.txt"
DIAGNOSTICS_FILE="test-bed/cf-diag-log-corp-loans.txt"

cargo run --release -- \
--as-on-date 28-02-2022 \
--input-master-file-path ${INPUT_MASTER} \
--input-cashflow-file-path ${INPUT_CASHFLOW} \
--input-ledger-file-path ${INPUT_LEDGER} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
# --log-level trace \
# --diagnostics-flag true \

