#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
OUTPUT=$"test-bed/ecl-rpt1"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
REQ_FIELDS_FILE=$"test-bed/req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"

/home/pavan/Work/tempdir/SuperDB-Batch/hdfc/tools/ecl/target/release/ecl \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--source Finnone \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--as-on-date 06-08-2019
#--log-level trace \
#--diagnostics-flag true
