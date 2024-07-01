#!/usr/bin/env bash
INPUT_FILE=$"test-bed/input-cf.cf"
LLGRECON=$"test-bed/llg-recon.txt"
ALMFILE=$"test-bed/alm-master.xlsx"
OUTPUT=$"test-bed/Output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
METADATA_FILE=$"test-bed/metadata.json"
REQ_DATA_FILE=$"test-bed/req.json"
EXRT_RATE_FILE=$"test-bed/Exchange-Rate.txt"
GL_MASTER_FILE=$"test-bed/gl-master.xlsx"

cargo run --release -- \
--input-file-path ${INPUT_FILE} \
--llg-recon-file-path ${LLGRECON} \
--alm-master-file-path ${ALMFILE} \
--alm-master-sheet-name Sheet1 \
--gl-master-file-path ${GL_MASTER_FILE} \
--gl-master-sheet-name Sheet1 \
--req-fields-file-path ${REQ_DATA_FILE} \
--exchange-rate-file ${EXRT_RATE_FILE} \
--output-file-path ${OUTPUT} \
--metadata-file-path ${METADATA_FILE} \
--base-currency INR \
--is-consolidated true \
--default-gl-code 999999 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 22-02-2022 \
--log-level trace \
--diagnostics-flag true


