#!/usr/bin/env bash

INPUT=$"test-bed/02012019/cf-ubs-loans-output.cf"
OUTPUT=$"test-bed/output.txt"
OPEN=$"test-bed/open.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
KNOWN_FIELDS_FILE=$"test-bed/a_w_cf_ubs_loans_req.json"
METADATA_FILE=$"test-bed/a_w_cf_ubs_loans_metadata.json"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--open-accounts-file ${OPEN} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--known-fields-file $KNOWN_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--as-on-date 24-01-2011 \
#--log-level trace \
#--diagnostics-flag true
