#!/usr/bin/env bash

INPUT_FILE=$"test-bed/TDCFOutput.cf"
REQ_FIELDS=$"test-bed/req_fields.json"
OUTPUT=$"test-bed/output_und.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MATURITY_BKT_FILE=$"test-bed/maturity_bkt_def_file_path.txt"
RULES_FILE=$"test-bed/rules.txt"
EXCHANGE_RATE=$"test-bed/exchange_rate.txt"
METADATA_FILE=$"test-bed/tdcf_metadata.json"

cargo run -- \
--input-file-path ${INPUT_FILE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-path ${REQ_FIELDS} \
--as-on-date 31-05-2023 \
--rules-file-path ${RULES_FILE} \
--diagnostics-flag true \
--bucket-scheme-id "99" \
--by-bucket-struct "N" \
--non-by-bucket-id "2" \
--mat-bkt-def-file-path ${MATURITY_BKT_FILE} \
--is-consolidated true \
--base-currency INR \
--exchange-rate-file-path ${EXCHANGE_RATE} \
--metadata-file-path ${METADATA_FILE} \
--default-llg-code 1111 \
--llg-id "5915,1111"
