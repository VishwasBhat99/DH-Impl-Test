#!/usr/bin/env bash
INPUT_FILE=$"test-bed/output.cf"
OUTPUT=$"test-bed/output.txt"
LOG_FILE="test-bed/log.txt"
DIAGLOG_FILE="test-bed/diag-log.txt"
REQ="test-bed/cf-to-txt-req-fields.json"
METADATA="test-bed/metadata.json"
EX_RATE="test-bed/1000ExchangeRate.txt"
RULES="test-bed/rules.txt"

cargo run --release -- \
--diagnostics-log-file ${DIAGLOG_FILE} \
--input-file-path ${INPUT_FILE} \
--log-file ${LOG_FILE} \
--metadata-file-path ${METADATA} \
--output-file-path ${OUTPUT} \
--required-fields-file-path ${REQ} \
--req-header false \
--default-overdue-llg 3000 \
--exchange-rate-file ${EX_RATE} \
--balm-rule-file-path ${RULES} \
--balm-default-llg 1999 \
--acc-currency INR \
--base-currency INR \
--as-on-mandatory true \
--as-on-date "01-09-2022" \
--log-level trace \
--diagnostics-flag false
