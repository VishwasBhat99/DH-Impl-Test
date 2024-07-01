#!/usr/bin/env bash

AS_ON_DATE=10-01-2020
CURRENCY_ID=INR
INPUT_FILE=$"test-bed/cust-bal.txt"
UCIC_MAP_FILE=$"test-bed/ucicid-masteri.txt"
OUTPUT_FILE=$"test-bed/pp-deposits"
EXRT_FILE=$"test-bed/1000ExchangeRate.txt"
LOG_FILE=$"test-bed/dep_log.txt"
DIAGNOSTICS_FILE=$"test-bed/dep_diag-log.txt"

rm -f ../test-bed/pp-deposits/*

cargo run --release -- \
--ason ${AS_ON_DATE} \
--top_cust_count 20 \
--country_code INR \
--currency_id ${CURRENCY_ID} \
--input_file ${INPUT_FILE} \
--ucic_map_file ${UCIC_MAP_FILE} \
--exrt_file ${EXRT_FILE} \
--output_file ${OUTPUT_FILE} \
--log_file ${LOG_FILE} \
--diag_log_file ${DIAGNOSTICS_FILE}
