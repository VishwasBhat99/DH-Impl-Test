#!/usr/bin/env bash

RET_CUST_FILE=$"test-bed/Ret_cust_aggr.txt"
MASTER_FILE=$"test-bed/master-file.txt"
DIS_SMRY_FILE=$"test-bed/CustID-wise-dis_smry.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--ret-cust-path ${RET_CUST_FILE} \
--master-file-path ${MASTER_FILE} \
--dis-smry-path ${DIS_SMRY_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date  31-01-2022 \
--log-level trace \
--diagnostics-flag false
