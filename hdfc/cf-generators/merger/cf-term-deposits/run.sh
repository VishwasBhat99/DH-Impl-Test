#!/usr/bin/env bash

INPUT="test-bed/td-output.cf"
OUTPUT="test-bed/td-output"
LOG_FILE="test-bed/td-log.txt"
DIAGNOSTICS_FILE="test-bed/td-diag-log.txt"
ALM_MASTER_FILE_PATH="test-bed/alm-master-file.txt"
COMMON_CUST_FILE_PATH="test-bed/common-cust-file.txt"
FINNONE_MASTER_FILE_PATH="test-bed/finnone-master-file.txt"
METADATA_FILE_PATH="test-bed/met.json"
NPA_CLASS_FILE_PATH="test-bed/npa-class-file.txt"
REQ_FIELDS_FILE_PATH="test-bed/required-fields-file.json"
RESID_FILE_PATH="test-bed/resid-file.txt"
RESTRUCTURE_FLAG_FILE_PATH="test-bed/restructure-flag-file.txt"
RISK_WEIGHT_FILE_PATH="test-bed/risk-weight-file.txt"
ORA_GL_FILE_PATH="test-bed/ora-gl-file.txt"
MASTER_LLG_FILE_PATH="test-bed/master-llg-file.txt"
ORA_GL_SHEET_NAME="ORA_GL_Sheet"
MASTER_LLG_SHEET_NAME="Master_LLG_Sheet"

cargo run --release -- \
--input-file "${INPUT}" \
--output-file "${OUTPUT}" \
--metadata-file-path "${METADATA_FILE_PATH}" \
--req-fields-file-path "${REQ_FIELDS_FILE_PATH}" \
--finnone-master-file-path "${FINNONE_MASTER_FILE_PATH}" \
--risk-weight-file-path "${RISK_WEIGHT_FILE_PATH}" \
--resid-file-path "${RESID_FILE_PATH}" \
--restructure-flag-file-path "${RESTRUCTURE_FLAG_FILE_PATH}" \
--common-cust-file-path "${COMMON_CUST_FILE_PATH}" \
--npa-class-file-path "${NPA_CLASS_FILE_PATH}" \
--log-file "${LOG_FILE}" \
--diagnostics-log-file "${DIAGNOSTICS_FILE}" \
--as-on-date "21-01-2021" \
--ora-gl-file-path "${ORA_GL_FILE_PATH}" \
--master-llg-file-path "${MASTER_LLG_FILE_PATH}" \
--ora-gl-sheet-name "${ORA_GL_SHEET_NAME}" \
--master-llg-sheet-name "${MASTER_LLG_SHEET_NAME}"
