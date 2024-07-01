#!/usr/bin/env bash
MAPPING_MASTER=$"test-bed/mapping_master.txt"
CAT_MASTER=$"test-bed/category_master.txt"
BKT_MASTER=$"test-bed/bucket_master.txt"
INPUT_FILE=$"test-bed/input2.txt"
OUTPUT_FILE=$"test-bed/output.txt"
LOG=$"test-bed/log.txt"
DIAGLOG=$"test-bed/diag-log.txt"

cargo run -- \
--as-on-date 30-06-2023 \
--bucket-master-file-path ${BKT_MASTER} \
--category-master-file-path ${CAT_MASTER} \
--currency INR \
--diagnostics-log-file ${DIAGLOG} \
--input-file-path ${INPUT_FILE} \
--log-file ${LOG} \
--mapping-master-file-path ${MAPPING_MASTER} \
--output-file-path ${OUTPUT_FILE}
#--log-level trace \
#--diagnostics-flag false
