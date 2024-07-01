#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"01-01-2019"
TO_DATE=$"31-01-2019"
INPUT=$"Input/cf-ubs-loans-output.cf"
M_RULE=$"Input/ubs_method_rules.txt"
BC_RULE=$"Input/ubs_bc_rules.txt"
ADJ_RULE_FIX=$"Input/rules/ubs_adj_rules_latest.txt"
ADJ_RULE_VAR=$"Input/rules/ubs_adj_rules_lastest_var.txt"
METADATA=$"Input/metadata_UBS.json"
BC_FILE=$"Input/Rates/"
EXCH_RATE_FILE=$"Input/Exch.txt"
FTP_RATES_FILE=$"Input/FTPRates.txt"
ADJ_RATES_FILE=$"Input/Rates/adj.txt"
AMB_FILE=$"Input/amb_ubs.csv"
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
--fix-adj-rule-file ${ADJ_RULE_FIX} \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--output-file ${OUTPUT} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--ftp-rates-file ${FTP_RATES_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--amb-file ${AMB_FILE} \
--default-method 1002 \
--default-basecurve 1110 \
--fixed-adjustments-count 3 \
--var-adjustments-count 1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
