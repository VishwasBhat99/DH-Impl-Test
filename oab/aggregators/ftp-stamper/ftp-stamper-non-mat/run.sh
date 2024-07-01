#!/usr/bin/env bash

FTP_RUN_ID=$"110228"
FROM_DATE=$"01-01-2019"
TO_DATE=$"31-01-2019"
INPUT=$"test-bed/stmp-ca.cf"
AVG_BAL_FILE=$"test-bed/avg-monthly-ca.txt"
REQ_FIELDS_FILE=$"test-bed/ftp-req-casaod.json"
METADATA=$"test-bed/metadata-casaod-stamper.json"
M_RULE=$"test-bed/m-rules-ca.txt"
BC_RULE=$"test-bed/bc-rules-ca.txt"
ADJ_RULE=$"test-bed/adj-rules-ca.txt"
BC_FILE=$"test-bed/Rates/"
EXCH_RATE_FILE=$"test-bed/1000ExchangeRate.txt"
#lock method
FTP_RATES_FILE=$"test-bed/ftp_rates.txt"
OUTPUT=$"test-bed/output/ftp-ca"
LOG_FILE=$"test-bed/output/ftp-log-ca.txt"
DIAGNOSTICS_FILE=$"test-bed/output/ftp-diag-log-ca.txt"

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
--default-method 1023 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
