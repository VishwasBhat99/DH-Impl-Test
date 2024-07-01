#!/usr/bin/env bash

MSF=$"test-bed/msf_master.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"


cargo run -- \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--msf-file ${MSF} \
--tot-gsec-bv 1000 \
--tot-gsec-mv  1000 \
--ndtl-val 1000 \
--excess-slr-val 1000 \
--slr-pct  5.0 \
--as-on-date 31-08-2021 
#--log-level trace \
#--diagnostics-flag true
