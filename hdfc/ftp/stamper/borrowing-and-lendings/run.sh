#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"2019-01-01"
TO_DATE=$"2019-01-01"
INPUT=$"Input/cf-ubs-loans-output.cf"
M_RULE=$"Input/ubs_method_rules.txt"
BC_RULE=$"Input/ubs_bc_rules.txt"
ADJ_RULE=$"Input/ubs_adj_rules.txt"
METADATA=$"Input/metadata_UBS.json"
BC_FILE=$"Input/Rates/"
EXCH_RATE_FILE=$"Input/Exch.txt"
FTP_RATES_FILE=$"Input/FTPRates.txt"
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
--default-method 1002 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
