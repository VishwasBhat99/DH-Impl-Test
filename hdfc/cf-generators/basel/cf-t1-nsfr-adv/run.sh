#!/usr/bin/env bash

INPUT=$"test-bed/input.cf"
EMPTY=$"test-bed/empty.txt"
METADATA=$"test-bed/metadata.json"
MASTER_FILE=$"test-bed/master_file.csv"
REQ=$"test-bed/req_fields.json"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2022 \
--log-level trace \
--diagnostics-flag true \
--slabs-file ${EMPTY} \
--cust-master-file ${EMPTY} \
--rw-master-file ${EMPTY} \
--restructured-flag-file-path ${EMPTY} \
--residential-mortgage-file-path ${EMPTY} \
--req-fields-file ${REQ} \
--account-metadata-file ${METADATA} \
--cust-file ${EMPTY} \
--cust-id-file ${EMPTY} \
--biu-file ${EMPTY} \
--class-file ${EMPTY} \
--ea-master-file ${MASTER_FILE} \
--has-cashflows true

