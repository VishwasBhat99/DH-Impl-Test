#!/usr/bin/env bash

AS_ON_DATE=$"17-02-2020"
INPUT=$"test-bed/OTHERBORROW_202.txt"
MASTER_LLG=$"test-bed/MappingMaster_v2.xlsx"
SHEET_NAME=$"Sheet1"
CUST_MASTER=$"test-bed/CustMaster.txt"
TREASURY_GL_MAP=$"test-bed/treasury_gl_map.txt"
CON=$"test-bed/output/concat.txt"
REC=$"test-bed/output/ReconRpt.txt"
OUT=$"test-bed/output/output.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--as-on-date ${AS_ON_DATE} \
--input-file ${INPUT} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name ${SHEET_NAME} \
--cust-master ${CUST_MASTER} \
--tresury-gl-map ${TREASURY_GL_MAP} \
--concat-file ${CON} \
--rec-output-file ${REC} \
--output-file ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE}
