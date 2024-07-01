#!/usr/bin/env bash

INPUT=$"test-bed/ret-cust-aggr/cust-master.txt"
BIU=$"test-bed/biu.txt"
CA=$"test-bed/ret-cust-aggr/ca-ret.txt"
SA=$"test-bed/ret-cust-aggr/sa-ret.txt"
RD=$"test-bed/RDEmpty.txt"
TD=$"test-bed/ret-cust-aggr/td-ret.txt"
OUTPUT=$"test-bed/output"
BKT=$"test-bed/Ca_BKT_Schema.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--biu-file $BIU \
--ca-file $CA \
--sa-file $SA \
--td-file $TD \
--rd-file $RD \
--output-file ${OUTPUT} \
--bkt-file ${BKT} \
--base-currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--max-stable-amt 500000 \
--as-on-date 30-06-2020 \
# --log-level debug \
# --diagnostics-flag true
