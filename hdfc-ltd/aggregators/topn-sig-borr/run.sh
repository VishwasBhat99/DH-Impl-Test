#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUTPUT=$"test-bed/output.txt"
EX_RATE=$"test-bed/exrate.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
NO_OF_ACCS=10
LIAB_VAL=10000000

cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT} \
--liab-val ${LIAB_VAL} \
--sig-perc 1.00 \
--ex-rate-file ${EX_RATE} \
--base-ccy "INR" \
--country-code "IND" \
--as-on-date 31-08-2021 \
#--log-level trace \
#--diagnostics-flag true

sort  -k7,7 -r -n -t $'|' test-bed/output.txt | head -$NO_OF_ACCS > test-bed/sig_top10_brw.txt
