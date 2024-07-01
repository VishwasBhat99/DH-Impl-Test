#!/usr/bin/env bash

INPUT=$"test-bed/TD30-11.csv"
OUT=$"test-bed/output.txt"
CON=$"test-bed/concat.txt"
REC=$"test-bed/ReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/MappingMaster1.xlsx"
CUST_MASTER=$"test-bed/Cust_master.txt"
OVERDUE_ACC=$"test-bed/TD_Overdue-FTP.txt"
cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--concat-file ${CON} \
--rec-output-file ${REC} \
--overdue-acc-file ${OVERDUE_ACC} \
--cust-master ${CUST_MASTER} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 30-11-2019 \
--log-level trace \
--diagnostics-flag true \
