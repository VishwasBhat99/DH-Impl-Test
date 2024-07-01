#!/usr/bin/env bash

INPUT=$"test-bed/Memberwise_ISIN_Holding_Statement_04Aug20.csv"
SEC=$"test-bed/SECDEALDATA_30062020.csv"
OUT=$"test-bed/NSFR_HoldingPreprocoutput.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"


cargo run --release -- \
--input-file ${INPUT} \
--sec-deal-file ${SEC} \
--output-file ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
