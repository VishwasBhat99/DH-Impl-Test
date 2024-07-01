#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"01-01-2019"
TO_DATE=$"01-01-2019"
INPUT=$"test-bed/CFOutput.cf"
M_RULE=$"test-bed/finnone_method_rules.txt"
BC_RULE=$"test-bed/finnone_bc_rules.txt"
ADJ_RULE_FX=$"test-bed/adj_latest_fix.txt"
ADJ_RULE_VAR=$"test-bed/adj_latest_var.txt"
METADATA=$"test-bed/finnone-metadata.json"
BC_FILE=$"test-bed/bc"
EXCH_RATE_FILE=$"test-bed/Exch.txt"
FTP_RATES_FILE=$"test-bed/FTP_Rates_fn.txt"
ADJ_RATES_FILE=$"test-bed/adj.txt"
AMB_FILE=$"test-bed/fn_amb.csv"
ADJ_CONFIG_FILE=$"test-bed/adj-config-file.txt"
OUTPUT=$"test-bed/output/FTPCFOutput"
REQ_FILE=$"test-bed/req-fields-finnone.json"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag_log.txt"

cargo run --release -- \
--ftp-runid ${FTPRUNID} \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--fix-adj-rule-file ${ADJ_RULE_FX} \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--output-file ${OUTPUT} \
--req-fields-file ${REQ_FILE} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--ftp-rates-file ${FTP_RATES_FILE} \
--amb-file ${AMB_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--adj-config-file ${ADJ_CONFIG_FILE} \
--default-method 1034 \
--default-basecurve 10 \
--fixed-adjustments-count 3 \
--var-adjustments-count 3 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--bal-precision 4 \
--rate-precision 4 \
--as-on-date 31-03-2023
#--log-level trace \
#--diagnostics-flag true
