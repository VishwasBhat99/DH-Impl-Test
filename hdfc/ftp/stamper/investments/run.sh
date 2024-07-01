#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"01-06-2023"
TO_DATE=$"30-06-2023"
INPUT=$"test-bed/cf-ubs-loans-output.cf"
M_RULE=$"test-bed/ubs_method_rules.txt"
BC_RULE=$"test-bed/ubs_bc_rules.txt"
ADJ_RULE=$"test-bed/ubs_adj_rules_latest.txt"
METADATA=$"test-bed/metadata_UBS.json"
BC_FILE=$"test-bed/Rates/"
EXCH_RATE_FILE=$"test-bed/Exch.txt"
FTP_RATES_FILE=$"test-bed/FTPRates.txt"
ADJ_RATES_FILE=$"test-bed/Rates/adj.txt"
OUTPUT=$"test-bed/output/FTPCFOutput"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag_log.txt"
AMB_FILE=$"test-bed/amb_file.xlsx"
REQ_FIELDS_FILE=$"test-bed/req_fields.json"

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
--amb-file ${AMB_FILE} \
--amb-sheet-name "Sheet1" \
--req-fields-file ${REQ_FIELDS_FILE} \
--default-method 1002 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
