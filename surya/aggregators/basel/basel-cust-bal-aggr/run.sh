#!/usr/bin/env bash

INPUT=$"test-bed/cust-master.txt"
BIU=$"test-bed/biu.txt"
CA=$"test-bed/ca-ret.txt"
SA=$"test-bed/sa-ret.txt"
RD=$"test-bed/rd-ret.txt"
TD=$"test-bed/td-ret.txt"
OUTPUT=$"test-bed/output"
BKT=$"test-bed/bkt-schema.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
LIEN=$"test-bed/lien.txt"

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
--is-casa-stable-amount true \
--as-on-date 30-06-2020 \
--is-lcy-or-ccy LCY \
--lien-file ${LIEN} \
# --log-level debug \
# --diagnostics-flag true
