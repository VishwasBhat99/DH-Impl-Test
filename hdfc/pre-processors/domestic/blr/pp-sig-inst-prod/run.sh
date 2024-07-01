#!/usr/bin/env bash

INPUT=$"test-bed/ledbal.csv"
OUT=$"test-bed/output.txt"
LIABILITY_BAL_FILE=$"test-bed/liability-bal.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/ProdMapMaster.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--prod-map-master ${MASTER} \
--sig_perc 1.0 \
--liability_bal_file ${LIABILITY_BAL_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
#--log-level trace \
#--diagnostics-flag true \
