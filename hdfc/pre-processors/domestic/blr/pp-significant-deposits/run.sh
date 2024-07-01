#!/usr/bin/env bash

AS_ON_DATE=10-01-2020
COUNTRY_CODE=IND
CURRENCY_ID=INR
TOP_CUST_COUNT=500
INPUT_FILE=$"test-bed/cust-bal.txt"
UCIC_MAP_FILE=$"test-bed/ucicid-masteri.txt";
OUTPUT_FILE=$"test-bed/output/pp-deposits.txt"
LOG_FILE=$"test-bed/output/dep_log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/dep_diag-log.txt"

rm -f ../test-bed/pp-deposits/*

cargo run --release -- \
--ason ${AS_ON_DATE} \
--country_code ${COUNTRY_CODE} \
--currency_id ${CURRENCY_ID} \
--dep_file ${INPUT_FILE} \
--ucic_map_file ${UCIC_MAP_FILE} \
--top_cust_count ${TOP_CUST_COUNT} \
--output_file ${OUTPUT_FILE} \
--log_file ${LOG_FILE} \
--diag_log_file ${DIAGNOSTICS_FILE}
