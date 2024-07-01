#!/usr/bin/env bash

INPUT_FILE=$"test-bed/SURYA_CONTRACT"
OUTPUT=$"test-bed/cf-ubs-loans-output.txt"
LOG_FILE=$"test-bed/cf-ubs-loans-log.txt"
CASHF=$"test-bed/SURYA_CASHFLOW"
DIAGNOSTICS_FILE=$"test-bed/cf-ubs-loans-diag-log.txt"
NCD=$"test-bed/NCDBenposTreasury_20240131.xlsx"
CP=$"test-bed/CP_BenposTreasury_20240131.xlsx"
PRODUCT_FILE=$"test-bed/SAP_Product_Type_2.xlsx"

cargo run -- \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--benpos-cashflow-file ${CASHF} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--cp-benpos-file ${CP} \
--input-master-file ${INPUT_FILE} \
--ncd-benpos-file ${NCD} \
--product-sheet-name "Sheet1" \
--product-file-path ${PRODUCT_FILE} \
--product-ids "7000" \
--ncd-benpos-sheet-name "Sheet1" \
--cp-benpos-sheet-name "Sheet1" \
--as-on-date 31-01-2024 \
--display-ccy "INR" \

