#!/usr/bin/env bash

FTPRUNID=$"052021"
FROM_DATE=$"01-05-2021"
TO_DATE=$"01-01-2019"
INPUT=$"test-bed/TDCFOutput.cf"
M_RULE=$"test-bed/method_rule.txt"
BC_RULE=$"test-bed/bc_rule.txt"
ADJ_RULE_FX=$"test-bed/adj_fix_rule.txt"
ADJ_RULE_VAR=$"test-bed/adj_var_rule.txt"
METADATA=$"test-bed/metadata.json"
BC_FILE=$"test-bed/"
EXCH_RATE_FILE=$"test-bed/exchangeRate.txt"
FTP_RATES_FILE=$"test-bed/FTPRates1111.txt"
ADJ_RATES_FILE=$"test-bed/adj.txt"
AMB_FILE=$"test-bed/fn_amb.csv"
AGGR_FILE=$"test-bed/aggr.txt"
OUTPUT=$"test-bed/Output/FTPCFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--ftp-runid ${FTPRUNID} \
--from-date ${FROM_DATE} \
--to-date 31-05-2021 \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--fix-adj-rule-file ${ADJ_RULE_FX} \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--output-file ${OUTPUT} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--ftp-rates-file ${FTP_RATES_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--amb-file ${AMB_FILE} \
--aggr-file ${AGGR_FILE} \
--default-method 1011 \
--default-basecurve 10 \
--fixed-adjustments-count 5 \
--var-adjustments-count 3 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true

# ftp_rates_file_path - not needed
