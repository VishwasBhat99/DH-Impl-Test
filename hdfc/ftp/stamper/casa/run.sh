#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"01-05-2019"
TO_DATE=$"31-05-2019"
INPUT=$"Input/output.cf"
M_RULE=$"Input/method_rules.txt"
BC_RULE=$"Input/CASA_RULES.txt"
ADJ_RULE=$"Input/adj_rules_latest.txt"
METADATA=$"Input/meta.json"
BC_FILE=$"Input/Rates/"
EXCH_RATE_FILE=$"Input/Exch.txt"
FTP_RATES_FILE=$"Input/FTPRates.txt"
ADJ_RATES_FILE=$"Input/Rates/adj.txt"
OUTPUT=$"output/FTPCFOutput"
LOG_FILE=$"output/log.txt"
DIAGNOSTICS_FILE=$"output/diag_log.txt"

cargo run --release -- \
--ftp-runid ${FTPRUNID} \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--adj-rule-file ${ADJ_RULE} \
--output-file ${OUTPUT} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--ftp-rates-file ${FTP_RATES_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--default-method 1023 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
