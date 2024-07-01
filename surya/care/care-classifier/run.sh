#!/usr/bin/env bash
INPUT_FILE=$"test-bed/input.cf"
METADATA_FILE=$"test-bed/metadata.json"
REQ_ACC_FIELD=$"test-bed/req.json"
RULES_FILE=$"test-bed/rules.txt"
OUTPUT_FILE=$"test-bed/output.cf"
RECON_FILE=$"test-bed/recon.txt"
MASTER_FILE=$"test-bed/master.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/dialog.txt"

cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--metadata-file-path ${METADATA_FILE} \
--req_field_file ${REQ_ACC_FIELD} \
--rules-file-path ${RULES_FILE} \
--output-file-path ${OUTPUT_FILE} \
--recon-file-path ${RECON_FILE} \
--master-file-path ${MASTER_FILE} \
--log-file ${LOG_FILE} \
--write-master true \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
