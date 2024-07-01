#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/input.txt"
MASTER_FILE=$"test-bed/master_llg_updated.xlsx"
EX=$"test-bed/1000ExchangeRate.txt"
OUTPUT_FILE=$"test-bed/output.txt"
REQ_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE="test-bed/metadata.json"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--alm-master-sheet-name "Sheet1" \
--exchange-rate-file ${EX} \
--base-currency "INR" \
--output-file ${OUTPUT_FILE} \
--req-fields-file ${REQ_FIELDS_FILE} \
--metadata-file ${METADATA_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
#--diagnostics-flag false
