#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
METADATA=$"test-bed/metadata.json"
REQ=$"test-bed/req_fields.json"
SLAB=$"test-bed/slab.txt"
CUST_MASTER=$"test-bed/cust_master.txt"
CUST=$"test-bed/cust.txt"
CUSTID=$"test-bed/custid.txt"
BIU=$"test-bed/biu.txt"
CLASS=$"test-bed/class.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--slabs-file ${SLAB} \
--cust-master-file ${CUST_MASTER} \
--cust-master-sep "|" \
--cust-sep "|" \
--cust-id-sep "|" \
--req-fields-file ${REQ} \
--account-metadata-file ${METADATA} \
--cust-file ${CUST} \
--cust-id-file ${CUSTID} \
--biu-file ${BIU} \
--class-file ${CLASS} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 
