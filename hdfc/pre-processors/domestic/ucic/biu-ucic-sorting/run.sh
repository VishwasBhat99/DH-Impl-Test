#!/usr/bin/env bash

UCIC_MASTER_FILE=$"test-bed/BIU_UCIC.txt"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--ucic-master-file ${UCIC_MASTER_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ucic-field-delimiter "|" \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
