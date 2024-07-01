#!/usr/bin/env bash

INP_FILE=$"test-bed/input.txt"
RES_FILE=$"test-bed/repayment_schedule.txt"
OP_FILE=$"test-bed/op"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
BORM_FILE=$"test-bed/BORM.txt"

cargo run --release -- \
--input-file-path ${INP_FILE} \
--repayment-schedule-file-path ${RES_FILE} \
--output-file-path ${OP_FILE} \
--as-on-date "30-09-2020" \
--od-additional-day 7 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--add-borm-file-path ${BORM_FILE} \
--log-level trace \
--diagnostics-flag false
