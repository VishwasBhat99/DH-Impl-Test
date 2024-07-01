#!/usr/bin/env bash

INPUT_FILE=$"test-bed/"
EXCHANGE_RATE_FILE=$"test-bed/"
COL_FILE=$"test-bed/"
METADATA_FILE=$"test-bed/"
OUTPUT_FILE=$"test-bed/"
REQ_FIELDS_FILE=$"test-bed/"
RULES_FILE=$"test-bed/"
SOURCE_FILE=$"test-bed/"
LOG_FILE=$"test-bed/"
DIAGNOSTICS_FILE=$"test-bed/"

cargo run --release -- \
--as-on-date 30-06-2023 \
--base-currency "INR" \
--ccy-mm-hc-prcnt 15 \
--col-file-path ${COL_FILE} \
--default-risk-weight 35 \
--default-sub-claim-id 55 \
--exchange-rate-file ${EXCHANGE_RATE_FILE} \
--input-file-path ${INPUT_FILE} \
--mat-mm-hc-prcnt 45 \
--metadata-file-path ${METADATA_FILE} \
--output-file-path ${OUTPUT_FILE} \
--req-fields-file-path ${REQ_FIELDS_FILE} \
--rules-file-path ${RULES_FILE} \
--src-file-name ${SOURCE_FILE} \
--log-file ${LOG_FILE} \
--is-consolidated true \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
