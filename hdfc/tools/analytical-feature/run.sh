#!/usr/bin/env bash

RETAIL_INPUT=$"test-bed/wd-ret-input.txt"
NON_RETAIL_INPUT=$"test-bed/wd-non-ret-input.txt"
OUTPUT=$"test-bed/Analytical-feature-output.txt"
EDW_ALM_CUSTOMER=$"test-bed/Customer_File.txt"
BIU=$"test-bed/BIU.txt"
MASTER_PROD_CUST_TYPE=$"test-bed/Master_file_cust_prod.xlsx"
MASTER_PROD_CUST_TYPE_FILE_SHEET="Custtype"
RUNOFF=$"test-bed/RunOff"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--retail_input_file ${RETAIL_INPUT} \
--non_retail_input_file ${NON_RETAIL_INPUT} \
--output_file ${OUTPUT} \
--edw_alm_customer_file ${EDW_ALM_CUSTOMER} \
--biu_file ${BIU} \
--master_prod_cust_type_file ${MASTER_PROD_CUST_TYPE} \
--master_prod_cust_type_file_sheet ${MASTER_PROD_CUST_TYPE_FILE_SHEET} \
--runoff_file ${RUNOFF} \
--log_file ${LOG_FILE} \
--diagnostics_log_file ${DIAGNOSTICS_FILE} \