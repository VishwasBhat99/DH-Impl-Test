#!/usr/bin/env bash

INPUT="test-bed/input.txt"
OUTPUT="test-bed/td-output"
LOG_FILE="test-bed/td-log.txt"
DIAGNOSTICS_FILE="test-bed/td-diag-log.txt"
ALM_MASTER_FILE_PATH="test-bed/alm-master-file.txt"
COMMON_CUST_FILE_PATH="test-bed/common-cust-file.txt"
FINNONE_MASTER_FILE_PATH="test-bed/finnone-master-file.txt"
METADATA_FILE_PATH="test-bed/metadata-file.json"
NPA_CLASS_FILE_PATH="test-bed/npa-class-file.txt"
REQ_FIELDS_FILE_PATH="test-bed/required-fields-file.json"
RESID_FILE_PATH="test-bed/resid-file.txt"
RESTRUCTURE_FLAG_FILE_PATH="test-bed/restructure-flag-file.txt"
RISK_WEIGHT_FILE_PATH="test-bed/risk-weight-file.txt"

cargo run --release -- \
--input-file "${INPUT}" \
--output-file "${OUTPUT}" \
--metadata-file-path "${METADATA_FILE_PATH}" \
--req-fields-file-path "${REQ_FIELDS_FILE_PATH}" \
--alm-master-file-path "${ALM_MASTER_FILE_PATH}" \
--finnone-master-file-path "${FINNONE_MASTER_FILE_PATH}" \
--risk-weight-file-path "${RISK_WEIGHT_FILE_PATH}" \
--resid-file-path "${RESID_FILE_PATH}" \
--restructure-flag-file-path "${RESTRUCTURE_FLAG_FILE_PATH}" \
--common-cust-file-path "${COMMON_CUST_FILE_PATH}" \
--npa-class-file-path "${NPA_CLASS_FILE_PATH}" \
--log-file "${LOG_FILE}" \
--diagnostics-log-file "${DIAGNOSTICS_FILE}" \
--log-level trace \
--as-on-date "21-01-2021" \
--diagnostics-flag true 