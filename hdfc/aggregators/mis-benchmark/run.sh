#!/usr/bin/env bash

OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
TENOR_FILE=$"test-bed/mis_benchmark_r1_tenor.txt"
CONFIG_FILE=$"test-bed/config_mis_benchmark_sls.json"

cargo run --release -- \
--bkt-scheme "1D,3D,5D,7D,9D,11D,30M1D" \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--bkt-scheme-id SLS \
--config-file $CONFIG_FILE \
--consol-ccy INR \
--org-tenor-file-path $TENOR_FILE \
--report-id R1 \
--as-on-date 01-01-2019 
#--log-level trace \
#--diagnostics-flag true
