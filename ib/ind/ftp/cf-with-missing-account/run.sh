#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-cc-non-emi.cf"
OUTPUT=$"test-bed/cf-non-emi-output_30092023"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REQUIRED_FIILDS_FILE=$"test-bed/cc_non_emi_closed_accs_req_fields.json"
METADATA_FILE=$"test-bed/cf-cc-nonemi-metadata.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-09-2023 \
--input-metadata-file ${METADATA_FILE} \
--required-fields-file ${REQUIRED_FIILDS_FILE} \
--date-prefix "test-bed/" \
--date-suffix ".cf" \
#--log-level trace \
#-diagnostics-flag true
