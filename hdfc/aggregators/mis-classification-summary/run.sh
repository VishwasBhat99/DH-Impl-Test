#!/usr/bin/env bash

CUST_FILE=$"test-bed/cust-aggr.txt"
RET_CUST_FILE=$"test-bed/Ret-cust-aggr.txt"
INPUT_FILE=$"test-bed/input.txt"
MASTER_FILE=$"test-bed/master-file.txt"
OP_FILE=$"test-bed/Summary-cust-aggr.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
COUNTRY="IND"
rm $INPUT_FILE
cat $CUST_FILE >>$INPUT_FILE
cat $RET_CUST_FILE >> $INPUT_FILE
cargo run --release -- \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--country_name ${COUNTRY} \
--as-on-date 21-02-2022 \
--output-file ${OP_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
