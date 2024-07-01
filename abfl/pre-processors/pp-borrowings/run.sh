#!/usr/bin/env bash

INPUT_FILE=$"test-bed/alm-borrowing-master.txt"
ALM_CASHFLOW=$"test-bed/alm-cashflow.txt"
BENPOS_DATA=$"test-bed/benpos-file-new.xlsx"
BENPOS_MAPPING=$"test-bed/benpos-mapping-master.xlsx"
FLOATING_MAPPING=$"test-bed/floating-mapping-master.xlsx"
BORROWING_UPDATETYPE_MASTER=$"test-bed/borrowing-updatetype-master.xlsx"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--input-file ${INPUT_FILE} \
--input-cf-file ${ALM_CASHFLOW} \
--benpos-data-file ${BENPOS_DATA} \
--benpos-mapping-file ${BENPOS_MAPPING} \
--floating-mapping-file ${FLOATING_MAPPING} \
--borrowing-update-type-master ${BORROWING_UPDATETYPE_MASTER} \
--benpos-data-sheet Input \
--benpos-mapping-sheet Sheet1 \
--floating-mapping-sheet Sheet1 \
--borrowing-update-type-master-sheet Sheet1 \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--benpos-col-count 75 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2022 \
--log-level debug \
--diagnostics-flag true
