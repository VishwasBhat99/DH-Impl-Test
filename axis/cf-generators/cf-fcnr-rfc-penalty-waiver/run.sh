#!/usr/bin/env bash

INP_FILE=$"test-bed/output_fcnr_rfc.txt"
OUTPUT_FILE=$"test-bed/cfoutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
CONFIG_FILE=$"test-bed/fcnr-config.txt"

cargo run -- \
--as-on-date 15-05-2023 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INP_FILE} \
--log-file ${LOG_FILE} \
--config-file ${CONFIG_FILE} \
--output-file ${OUTPUT_FILE} 
