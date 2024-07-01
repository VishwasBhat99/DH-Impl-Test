#!/usr/bin/env bash

OUTPUT=$"test-bed/summary.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
TENOR_FILE=$"test-bed/master/tenor_master.xlsx"
ALCO=$"test-bed/master/alco_master.xlsx"
PSL=$"test-bed/master/PSL_master.xlsx"
RATE=$"test-bed/master/rate_man.xls"
CONFIG_FILE=$"test-bed/config/config_master.json"

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file $CONFIG_FILE \
--consol-ccy INR \
--tenor-master $TENOR_FILE \
--alco-master $ALCO \
--psl-master $PSL \
--rate-bucket-master $RATE \
--rate-sheet 50bps \
--as-on-date 01-01-2019 \
--log-level trace \
--diagnostics-flag true
