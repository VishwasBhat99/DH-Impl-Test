#!/usr/bin/env bash

touch "test-bed/total_deposit_bal.csv"
cat "test-bed/summary_ret_less_stable.csv" >> "test-bed/total_deposit_bal.csv"
cat "test-bed/summary_ret_stable.csv" >> "test-bed/total_deposit_bal.csv"
cat "test-bed/summary_non_ret_stable.csv" >> "test-bed/total_deposit_bal.csv"
cat "test-bed/summary_non_ret_less_stable.csv" >> "test-bed/total_deposit_bal.csv"


SLS_FILE=$"test-bed/SLS.txt"
NWD_FILE=$"test-bed/NWD_CALC_02022023.csv"
TOT_DEP_BAL_FILE=$"test-bed/total_deposit_bal.csv"
OUTPUT=$"test-bed/summary-deposit-moc.csv"
LOG_FILE=$"test-bed/moc_dep_log.txt"
DIAGNOSTICS_FILE=$"test-bed/moc_dep_diaglog.txt"

cargo run --release -- \
--sls-file ${SLS_FILE} \
--output-file ${OUTPUT} \
--nwd-file ${NWD_FILE} \
--tot-dep-bal-file ${TOT_DEP_BAL_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--currency "INR" \
--country "IND" \
--amount "100000000000" \
--as-on-date 02-02-2023
#--log-level trace \
#--diagnostics-flag true
