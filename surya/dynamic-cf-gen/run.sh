#!/usr/bin/env bash

PRJ_MASTER=$"test-bed/coa-master.txt"
BM_DIS_FILE=$"test-bed/bm-dis.txt"
DAY_DIS_FILE=$"test-bed/day-dis.txt"
TENOR_DIS_FILE=$"test-bed/tenor-dis.txt"
BM_RATE_FILE=$"test-bed/bm-rates.txt"
OP_FILE=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--prj-coa "2000" \
--as-on-date 31-03-2021 \
--new-business-value 450000 \
--currency INR \
--interest-basis "Thirtyby360" \
--cf-type EMI \
--coa-master-file-path ${PRJ_MASTER} \
--disbursement-by-bm-file-path ${BM_DIS_FILE} \
--disbursement-by-day-file-path ${DAY_DIS_FILE} \
--disbursement-by-tenor-file-path ${TENOR_DIS_FILE} \
--bm-rates-file-path ${BM_RATE_FILE} \
--output-file-path ${OP_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
--diagnostics-flag false
