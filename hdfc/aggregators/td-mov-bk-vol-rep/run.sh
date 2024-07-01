#!/usr/bin/env bash

OUTPUT=$"test-bed/output/summary"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diaglog.txt"
CONFIG_FILE=$"test-bed/config/config_master.json"
ALCO=$"test-bed/master/alco_master.xlsx"
TENOR_FILE=$"test-bed/master/tenor_master.xlsx"
BUCKET=$"test-bed/master/bucket_master.xlsx"
CATEGORY=$"test-bed/master/categ_master.xlsx"
LCR=$"test-bed/master/lcr_master.txt"
WD_NWD=$"test-bed/master/wd_nwd_master.xlsx"

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file ${CONFIG_FILE} \
--consol-ccy INR \
--alco-master $ALCO \
--tenor-master $TENOR_FILE \
--bucket-master $BUCKET \
--cat-master $CATEGORY \
--lcr-master $LCR \
--wd-nwd-master $WD_NWD \
--as-on-date 22-10-2021 
#--log-level trace \
#--diagnostics-flag true
