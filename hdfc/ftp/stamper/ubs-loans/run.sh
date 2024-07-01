#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"01-01-2023"
TO_DATE=$"01-01-2023"
INPUT_FILE=$"test-bed/cf-ubs-loans-output.cf"
M_RULE=$"test-bed/ubs_method_rules.txt"
BC_RULE=$"test-bed/ubs_bc_rules.txt"
ADJ_RULE_FIX=$"test-bed/ubs_adj_rules_latest.txt"
ADJ_RULE_VAR=$"test-bed/ubs_adj_rules_latest_var.txt"
METADATA=$"test-bed/metadata_UBS.json"
BC_FILE=$"test-bed/bc.txt"
EXCH_RATE_FILE=$"test-bed/Exch.txt"
FTP_RATES_FILE=$"test-bed/FTPRates.txt"
ADJ_RATES_FILE=$"test-bed/adj.txt"
AMB_FILE=$"test-bed/amb_ubs.txt"
REQ_FILE=$"test-bed/req-fields-ubs.json"
ADJ_CONFIG_FILE=$"test-bed/adj_config.txt"
UBS_LOCK_FILE=$"test-bed/ubs_lock.txt"
OUTPUT=$"test-bed/FTPCFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"

cargo run --release -- \
--ftp-runid ${FTPRUNID} \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT_FILE} \
--meta-data-file ${METADATA} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--fix-adj-rule-file ${ADJ_RULE_FIX} \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--output-file ${OUTPUT} \
--req-fields-file ${REQ_FILE} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--ftp-rates-file ${FTP_RATES_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--adj-config-file ${ADJ_CONFIG_FILE} \
--ubs-lock-file ${UBS_LOCK_FILE} \
--amb-file ${AMB_FILE} \
--default-method 1002 \
--default-basecurve 1110 \
--fixed-adjustments-count 3 \
--var-adjustments-count 1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2024 \
--log-level trace \
--bal-precision 4 \
--rate-precision 4 \
--diagnostics-flag true
