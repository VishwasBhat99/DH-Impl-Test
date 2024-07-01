#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
REQ_FIELDS_FILE=$"test-bed/req.json"
METADATA_FILE=$"test-bed/metadata.json"
BALM_RATING=$"test-bed/balm-rating.txt"
SPREAD=$"test-bed/spread-rating.txt"
LLG_SPREAD=$"test-bed/llg-to-spread.txt"
RULES=$"test-bed/rules.txt"
DEF_VALUES=$"test-bed/def-values.json"


cargo run --release -- \
--input-file-path ${INPUT} \
--output-file-path ${OUTPUT} \
--log-file ${LOG_FILE} \
--balm-rating-file ${BALM_RATING} \
--spread-rate-file ${SPREAD} \
--llg-to-spread-mapper-file ${LLG_SPREAD} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-file ${REQ_FIELDS_FILE} \
--metadata-file-path ${METADATA_FILE} \
--rules-file-path ${RULES} \
--default-values-file ${DEF_VALUES} \
--as-on-date 30-06-2023 \
#--log-level trace \
#--diagnostics-flag true
