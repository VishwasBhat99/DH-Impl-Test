#!/usr/bin/env bash

INPUT=$"test-bed/non_emi.txt"
NPA_DATA=$"test-bed/npa_data_file.txt"
NPA_LIVE=$"test-bed/npa_live.txt"
NPA_CONFIG=$"test-bed/npa_config.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run -- \
--input-file ${INPUT} \
--npa-config-file ${NPA_CONFIG} \
--npa-data-file ${NPA_DATA} \
--npa-live-file ${NPA_LIVE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-05-2022
