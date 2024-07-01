#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
INPUT_FILE=$"test-bed/output.cf"
OUTPUT=$"test-bed/td"
MASTER=$"test-bed/master.txt"
METADATA=$"test-bed/metadata.json"
REQ_FIELDS=$"test-bed/req_fields.json"

cargo run -- \
--as-on-date 24-05-2024 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file-path ${INPUT_FILE} \
--log-file ${LOG_FILE} \
--master-file-path ${MASTER} \
--metadata-file-path ${METADATA} \
--output-file-path ${OUTPUT} \
--req-fields-file ${REQ_FIELDS} \
--log-level trace \
--diagnostics-flag false \
