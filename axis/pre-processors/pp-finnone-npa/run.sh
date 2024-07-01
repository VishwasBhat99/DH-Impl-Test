#!/usr/bin/env bash

INPUT=$"test-bed/UAT/npa_input.txt"
NPA_DATA=$"test-bed/npa_data.txt"
NPA_LIVE=$"test-bed/UAT/npa_live.txt"
NPA_CONFIG=$"test-bed/npa_config.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA=$"test-bed/pp_npa_finnone_metadata.json"
REQ_FIELDS=$"test-bed/pp_npa_finnone_req_fields.json"

cargo run --release -- \
--input-file ${INPUT} \
--npa-config-file ${NPA_CONFIG} \
--input-metadata-file ${METADATA} \
--req-fields-file-path ${REQ_FIELDS} \
--npa-data-file ${NPA_DATA} \
--npa-live-file ${NPA_LIVE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--cif-report test-bed/CIF_REPORT.txt \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 31-05-2022
