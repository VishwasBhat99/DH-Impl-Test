#!/usr/bin/env bash

INPUT=$"test-bed/citi_ca.cf"
OUTPUT=$"test-bed/out"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_META="test-bed/citi_ca_output_metadata.json"
OUTPUT_META="test-bed/basel_citi_casa_meta.json"


cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2022 \
--input-metadata-file ${INPUT_META} \
--output-metadata-file ${OUTPUT_META} \
--source-name 0014