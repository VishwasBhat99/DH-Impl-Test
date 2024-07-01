#!/usr/bin/env bash

FTP_RUN_ID=$"110227"
FROM_DATE=$"01-12-2018"
TO_DATE=$"31-12-2018"
INPUT=$"test-bed/stmp-loan-non-capitalized.cf"
AVG_BAL_FILE=$"test-bed/avg-monthly-loan-non-capitalized.txt"
REQ_FIELDS_FILE=$"test-bed/ftp-req-loan-non-capitalized.json"
METADATA=$"test-bed/metadata-loan-non-capitalized.json"
M_RULE=$"test-bed/m-rules-loan-non-capitalized.txt"
BC_RULE=$"test-bed/bc-rules-loan-non-capitalized.txt"
ADJ_RULE=$"test-bed/adj-rules-loan-non-capitalized.txt"
BC_FILE=$"test-bed/Rates/"
EXCH_RATE_FILE=$"test-bed/1000ExchangeRate.txt"
#lock method
FTP_RATES_FILE=$"test-bed/FTPRates.txt"
OUTPUT=$"test-bed/output/ftp-loan-non-capitalized"
LOG_FILE=$"test-bed/output/ftp-log-loan-non-capitalized.txt"
DIAGNOSTICS_FILE=$"test-bed/output/ftp-diag-log-loan-non-capitalized.txt"

rm -f test-bed/output/*

cargo run --release -- \
--ftp-run-id ${FTP_RUN_ID} \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--avg-bal-file ${AVG_BAL_FILE} \
--req-fields-file $REQ_FIELDS_FILE \
--meta-data-file ${METADATA} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--adj-rule-file ${ADJ_RULE} \
--output-file ${OUTPUT} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--src-local-ccy OMR \
--ftp-rates-file ${FTP_RATES_FILE} \
--default-method 1002 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
