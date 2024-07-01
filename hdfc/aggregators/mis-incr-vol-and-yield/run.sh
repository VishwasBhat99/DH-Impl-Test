#!/usr/bin/env bash

OUTPUT=$"test-bed/summary"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
TENOR_FILE=$"test-bed/tenor.xlsx"
CONFIG_FILE=$"test-bed/config_mis_benchmark_sls.json"
XYZ=$"test-bed/acc_skip.txt"
PSL_FILE=$"test-bed/tenor.xlsx"
ALCO_FILE=$"test-bed/tenor.xlsx"

cargo run --release -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file $CONFIG_FILE \
--consol-ccy INR \
--org-tenor-file-path $TENOR_FILE \
--as-on-date 01-01-2020 \
--incr-acc-file-path $XYZ \
--alco-map-file-path $ALCO_FILE \
--alco-map-sheet-name "Sheet1" \
--org-tenor-sheet-name "Sheet1" \
--psl-map-file-path $PSL_FILE \
--psl-map-sheet-name "Sheet3" 
#--log-level trace \
#--diagnostics-flag true
