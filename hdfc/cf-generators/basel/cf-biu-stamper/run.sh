#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
BIU=$"test-bed/biu.txt"
CUST=$"test-bed/cust-master.txt"
TTL=$"test-bed/ttl.txt"
MASTER_NEW=$"test-bed/master_new.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ttl-bal-file ${TTL} \
--cust-master-file ${CUST} \
--biu-file ${BIU} \
--text-desc-master-file ${MASTER_NEW} \
--as-on-date 31-01-2022 \
#--log-level trace \
#--diagnostics-flag true
