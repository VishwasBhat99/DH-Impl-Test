#!/usr/bin/env bash
INPUT_FILE=$"test-bed/input.cf"
INPUT_METADATA_FILE=$"test-bed/metadata.json"
TOT_BAL_METADATA_FILE=$"test-bed/metadata.json"
TOT_BAL_FILE_PATH=$"test-bed/input.cf"
REQ_ACC_FIELD=$"test-bed/req.json"
TOT_BAL_REQ_FIELD=$"test-bed/tot-bal-req.json"
RULES_FILE=$"test-bed/rules.txt"
OUTPUT_FILE=$"test-bed/output"
RECON_FILE=$"test-bed/recon.txt"
OUT_MASTER_FILE=$"test-bed/master.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/dialog.txt"

cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--req_field_file ${REQ_ACC_FIELD} \
--metadata-file-path ${INPUT_METADATA_FILE} \
--tot-bal-metadata-file-path ${TOT_BAL_METADATA_FILE} \
--tot-bal-key-field "acc_no" \
--tot-bal-file-path ${TOT_BAL_FILE_PATH} \
--tot-bal-rules-file-path ${RULES_FILE} \
--output-file-path ${OUTPUT_FILE} \
--recon-file-path ${RECON_FILE} \
--default-class-id 112 \
--master-file-path ${OUT_MASTER_FILE} \
--write-master true \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE}
# --log-level ${INPUT_FILE} \
# --diagnostics-flag ${INPUT_FILE} \
