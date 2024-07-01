#!/usr/bin/env bash

OUTPUT=$"test-bed/summary"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
NPA_PREV_FILE=$"test-bed/npa/NPA_prev_master.csv"
NPA_CURR_FILE=$"test-bed/npa/NPA_curr_master.csv"
TENOR_FILE=$"test-bed/npa/Org_Tenor.xlsx"
ALCO=$"test-bed/npa/ALM_Grouping_master.xlsx"
PSL=$"test-bed/npa/PSL_master.xlsx"
CONFIG_FILE=$"test-bed/npa/config_npa.json"

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config-file $CONFIG_FILE \
--consol-ccy INR \
--npa-prev-master $NPA_PREV_FILE \
--npa-curr-master $NPA_CURR_FILE \
--tenor-master $TENOR_FILE \
--alco-master $ALCO \
--psl-master $PSL \
--as-on-date 22-10-2021 
#--log-level trace \
#--diagnostics-flag true
