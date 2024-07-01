#!/usr/bin/env bash

INPUT=$"test-bed/cf-out-finnone.cf"
OUTPUT=$"test-bed/output.txt"
REQ_FIELD=$"test-bed/req_fields.json"
CF_FIELD_FILE=$"test-bed/fin_loans_metadata.json"

rm output.txt -r

cargo run --release -- \
--cf-file ${INPUT} \
--output-file ${OUTPUT} \
--required-field-file ${REQ_FIELD} \
--cf-field-file ${CF_FIELD_FILE} \
--as-on-date 27-11-2019 \
