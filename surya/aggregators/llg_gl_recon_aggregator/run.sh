#!/usr/bin/env bash

CONFIG_FILE=$"test-bed/config-file.json"
OUTPUT=$"test-bed/Output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/metadata.json"
REQ_DATA_FILE=$"test-bed/req.json"
EXRT_RATE_FILE=$"test-bed/exrt.txt"
RULES_DATA_FILE=$"test-bed/rules.txt"

cargo run --release -- \
--config-file-path ${CONFIG_FILE} \
--output-file-path ${OUTPUT} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--exchange-rate-file ${EXRT_RATE_FILE} \
--as-on-date 22-02-2022 \
--country IN \
--base-currency INR \
--log-level trace \
--diagnostics-flag true


