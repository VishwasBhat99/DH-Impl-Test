#!/usr/bin/env bash

INPUT="test-bed/ALM_ODCC_ACCOUNTS.txt"
OUTPUT="test-bed/output.txt"
MASTER_FILE="test-bed/master.txt"
LOG_FILE="test-bed/log.txt"
DIAGNOSTICS_FILE="test-bed/diag-log.txt"
REP_FILE="test-bed/mapping_master.xlsx"
REP_SHEET_NAME="Sheet1"

cargo run --release -- \
--input-file "${INPUT}" \
--master-file "${MASTER_FILE}" \
--output-file "${OUTPUT}" \
--log-file "${LOG_FILE}" \
--diagnostics-log-file "${DIAGNOSTICS_FILE}" \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-09-2023 \
--repricing-file-sheet-name $REP_SHEET_NAME \
--repricing-master-file $REP_FILE \
