#!/usr/bin/env bash

INPUT1=$"test-bed/input1.txt"
INPUT2=$"test-bed/input2.txt"
INPUT3=$"test-bed/input3.txt"
MASTER=$"test-bed/master.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date $2 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--from-bucket 2 \
--seasonal-dis-rules-path ${INPUT1} \
--distribution-rules-path ${INPUT2} \
--input-file-path ${INPUT3} \
--is-seasonal true \
--log-file ${LOG_FILE} \
--master-file-path ${MASTER} \
--output-file ${OUTPUT} \
--to-bucket 4