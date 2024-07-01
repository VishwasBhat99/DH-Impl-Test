#!/usr/bin/env bash

INPUT="test-bed/input-file.cf"
OUTPUT="test-bed/cf-out"
LOG_FILE="test-bed/cf-log.txt"
DIAGNOSTICS_FILE="test-bed/cf-diag-log.txt"
VP_NPA_FILE_PATH="test-bed/vp_npa.csv"
OD_STUDY_MASTER_FILE_PATH="test-bed/od-study-master.xlsx"
EXCHANGE_RATE_FILE_PATH="test-bed/1000ExchangeRate.txt"
METADATA_FILE_PATH="test-bed/metadata.json"
REQ_FIELDS_FILE_PATH="test-bed/req.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--metadata-file-path ${METADATA_FILE_PATH} \
--req-fields-file-path ${REQ_FIELDS_FILE_PATH} \
--vp-npa-master-file-path ${VP_NPA_FILE_PATH} \
--od-study-master-file-path ${OD_STUDY_MASTER_FILE_PATH} \
--od-study-master-sheet-name "OD CC" \
--is-consolidated true \
--base-currency "INR" \
--exchange-rate-file-path ${EXCHANGE_RATE_FILE_PATH} \
--alm-line "OD-CC" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--as-on-date 21-01-2021 \
--diagnostics-flag true 
