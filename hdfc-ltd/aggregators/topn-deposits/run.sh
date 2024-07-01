#!/usr/bin/env bash

TD=$"test-bed/td_input.txt"
RD=$"test-bed/rd_input.txt"
OUTPUT=$"test-bed/output.txt"
EX_RATE=$"test-bed/exrate.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
NO_OF_ACCS=20

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--td-file ${TD} \
--rd-file ${RD} \
--ex-rate-file ${EX_RATE} \
--base-ccy "INR" \
--country-code "IND" \
--as-on-date 31-08-2021 \
#--log-level trace \
#--diagnostics-flag true

sort  -k11,11 -r -n -t $'|' test-bed/output.txt | head -$NO_OF_ACCS > test-bed/top20_dep.txt
