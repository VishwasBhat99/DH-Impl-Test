#!/usr/bin/env bash

INPUT=$"test-bed/CF/CASA/CFOutput.cf"
OUTPUT=$"test-bed/CF/CASA/casa"
NWD=$"test-bed/CF/CASA/nwd-file.txt"
BKT=$"test-bed/CF/CASA/bkt-schema.txt"
LOG_FILE=$"test-bed/CF/CASA/log.txt"
DIAGNOSTICS_FILE=$"test-bed/CF/CASA/diaglog.txt"
REQ_FIELDS_FILE=$"test-bed/CF/CASA/req_fields.json"
METADATA_FILE=$"test-bed/CF/CASA/metadata.json"
RET_CUST_TYPES=$"F,I,M,O,Q,V,Y,Z,R,2,3,4,5,8,9,11,12"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
CUST_MASTER=$"test-bed/cust-master.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--nwd-file ${NWD} \
--bkt-file ${BKT} \
--base-currency INR \
--exchange-rate-file $CURRENCY_CONV_FILE \
--log-file ${LOG_FILE} \
--ret-cust-types $RET_CUST_TYPES \
--cust-master-file $CUST_MASTER \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--check-wd false \
--is-consolidated true \
--as-on-date 01-09-2019
#--log-level trace \
#--diagnostics-flag true
