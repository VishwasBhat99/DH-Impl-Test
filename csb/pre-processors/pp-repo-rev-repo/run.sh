#!/usr/bin/env bash

INPUT=$"test-bed/REPOREVREPO_152.txt"
OUT=$"test-bed/output.txt"
CON=$"test-bed/concat.txt"
REC=$"test-bed/ReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/MappingMaster1.xlsx"
CUST_MASTER=$"test-bed/CustMaster.txt"
TREAS=$"test-bed/TreasuryMaster.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--concat-file ${CON} \
--rec-output-file ${REC} \
--cust-master ${CUST_MASTER} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--treas-gl-master ${TREAS} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
