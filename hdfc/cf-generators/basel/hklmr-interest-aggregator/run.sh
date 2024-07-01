#!/usr/bin/env bash

#awk -F "," '{print $1","$8}' "/UAT_SH_PREPROCESSDATA/HK/$1/Borrowings.txt" > "$PREPROCESS/HK/$1/borr-cust-master-int-aggr.txt"

CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/int_aggr_borr_log.txt"
DIAGNOSTICS_FILE=$"test-bed/int_aggr_borr_diag_log.txt"

#/home/dbuser/programs/HK/scripts/hklmr/config_modifier.sh $1 int_aggr_config_borr.json

cargo run --release -- \
--config-file-path ${CONFIG} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 31-08-2020 \
--log-level trace \
--diagnostics-flag false 
