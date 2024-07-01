#!/usr/bin/env bash
INPUT_FILE=$"test-bed/OD_CFoutput.cf"
OUTPUT=$"test-bed/output.txt"
LOG_FILE="test-bed/log.txt"
DIAGLOG_FILE="test-bed/diag-log.txt"
REQ="test-bed/req-field.json"
METADATA="test-bed/cf-od-metadata.json"
EX_RATE="test-bed/1000ExchangeRate.txt"
RULES="test-bed/rules.txt"
CUST="test-bed/customer_metadata.json"
CUST_MASTER="test-bed/cust.txt"
CONFIG="test-bed/config.json"

cargo run --release -- \
--diagnostics-log-file ${DIAGLOG_FILE} \
--input-file-path ${INPUT_FILE} \
--log-file ${LOG_FILE} \
--metadata-file-path ${METADATA} \
--output-file-path ${OUTPUT} \
--required-fields-file-path ${REQ} \
--exchange-rate-file ${EX_RATE} \
--balm-rule-file-path ${RULES} \
--balm-default-llg 1999 \
--acc-currency INR \
--base-currency INR \
--as-on-date "01-09-2022" \
--log-level trace \
--config-file-path ${CONFIG} \
--customer-master-file ${CUST_MASTER} \
--customer-master-metadata-file ${CUST} \
--diagnostics-flag false
