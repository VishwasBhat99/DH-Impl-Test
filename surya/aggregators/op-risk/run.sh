#!/usr/bin/env bash

LOG_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/test-bed/log.txt"
DIAGNOSTICS_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/test-bed/diaglog.txt"
INPUT_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/test-bed/GLCFOutput-10000034.cf"
OUTPUT_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/test-bed/output.txt"
REQUIRED_FIELDS_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/input-resources/gl_op_req_fields.json"
RULES_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/input-resources/gl_op_rules.txt"
CURRENCY_CONV_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/1000ExchangeRate.txt"
METADATA_FILE=$"/home/dell/Documents/SuperDB-Batch/surya/aggregators/op-risk/input-resources/gl_metadata.json"

cargo run --release -- \
--as-on-date 31-01-2021 \
--base-currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--log-level trace \
--default-llg-code 10009999 \
--rules-file ${RULES_FILE} \
--req-field-file ${REQUIRED_FIELDS_FILE} \
--exchange-rate-file ${CURRENCY_CONV_FILE} \
--metadata-file ${METADATA_FILE} \
--is-absolute false
