#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
LST=$"test-bed/lst.txt"
CF_METADATA_FILE=$"test-bed/cf_metadata.json"
LST_METADATA_FILE=$"test-bed/lst_metadata.json"
output=$"test-bed/output"
CONFIG=$"test-bed/config.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"

cargo run -- \
--input-cf-file-path ${INPUT} \
--input-lst-file-path ${LST} \
--output-file1-name _cf \
--output-file2-name _lst \
--output-file3-name _diff \
--output-file4-name _match \
--cf-metadata-file-path ${CF_METADATA_FILE} \
--lst-metadata-file-path ${LST_METADATA_FILE} \
--base-output-file ${output} \
--config-file-path ${CONFIG} \
--as-on-date 30-10-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true
