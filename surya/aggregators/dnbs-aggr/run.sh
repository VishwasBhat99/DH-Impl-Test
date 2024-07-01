#!/usr/bin/env bash

OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXCHANGE_RATE_FILE=$"test-bed/1000ExchangeRate.txt"
INPUT_FILE_PATH=$"test-bed/cf-out-td.cf"
METADATA_FILE_PATH=$"test-bed/metadata-td.json"
RULES_PATH=$"test-bed/empty.txt"
REQ_FIELD_FILE=$"test-bed/req.json"

cargo run --release -- \
--input-file ${INPUT_FILE_PATH} \
--metadata-file ${METADATA_FILE_PATH} \
--rules-file ${RULES_PATH} \
--req-fields-file ${REQ_FIELD_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--default-llg-id  "888" \
--as-on-date 30-06-2023 \
--log-level trace \
--diagnostics-flag true
