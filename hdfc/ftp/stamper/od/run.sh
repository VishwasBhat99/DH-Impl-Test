#!/usr/bin/env bash

FROM_DATE=$"01-06-2023"
TO_DATE=$"30-06-2023"
INPUT=$"test-bed/CFOutput.cf"
M_RULE=$"test-bed/method_rules.txt"
BC_RULE=$"test-bed/od_core_bc_rules.txt"
ADJ_RULE_FIX=$"test-bed/od_core_adj_rules_fix.txt"
ADJ_RULE_VAR=$"test-bed/od_core_adj_rules_var.txt"
METADATA=$"test-bed/od_metadata.json"
BC_FILE=$"test-bed/BMRates/"
EXCH_RATE_FILE=$"test-bed/Exch.txt"
FTP_RATES_FILE=$"test-bed/FTPRates.txt"
ADJ_RATES_FILE=$"test-bed/adj.txt"
OUTPUT=$"test-bed/output/FTPCFOutput"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag_log.txt"
REQ_FIELDS=$"test-bed/req-fields.json"
ADJ_CONFIG=$"test-bed/adj-config-file.txt"
AMB=$"test-bed/amb.txt"

cargo run --release -- \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--req-fields-file ${REQ_FIELDS} \
--m-rule-file ${M_RULE} \
--bc-rule-file ${BC_RULE} \
--fix-adj-rule-file ${ADJ_RULE_FIX} \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--fixed-adjustments-count 3 \
--var-adjustments-count 3 \
--output-file ${OUTPUT} \
--bc-file ${BC_FILE} \
--exch-rate-file ${EXCH_RATE_FILE} \
--adj-rates-file ${ADJ_RATES_FILE} \
--default-method 1023 \
--default-basecurve 1110 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--adj-config-file-path ${ADJ_CONFIG} \
--amb-file-path ${AMB} \
--as-on-date 30-09-2023 \
--diagnostics-flag true
