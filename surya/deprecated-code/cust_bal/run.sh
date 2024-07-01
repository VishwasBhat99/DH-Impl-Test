#!/usr/bin/env bash

INPUT=$"test-bed/cust-master.txt"
BIU=$"test-bed/biu.txt"
CASA=$"test-bed/casa-ret-total.txt"
RD=$"test-bed/rd-ret-total.txt"
TD=$"test-bed/td-ret-total.txt"
OUTPUT=$"test-bed/output"
BKT=$"test-bed/bkt-schema.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--biu-file $BIU \
--casa-file $CASA \
--td-file $TD \
--rd-file $RD \
--output-file ${OUTPUT} \
--bkt-file ${BKT} \
--base-currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 01-09-2019 \
--log-level debug \
--diagnostics-flag true
