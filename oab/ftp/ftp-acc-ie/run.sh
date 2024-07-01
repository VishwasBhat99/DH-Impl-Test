#!/usr/bin/env bash

INPUT_FILE=$"test-bed/input.txt"
FTP_ACCIE=$"test-bed/FTPAccIE.txt"
OUTPUT=$"test-bed/output.txt"
EXRT=$"test-bed/exrt.txt"
METADATA=$"test-bed/metadata.json"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REQ_FILE=$"test-bed/req-fields.json"

cargo run  -- \
--input-file ${INPUT_FILE} \
--ftp-accie-file ${FTP_ACCIE} \
--output-file ${OUTPUT} \
--interest-income CA \
--interest-expense TD \
--log-file ${LOG_FILE} \
--exrt-file ${EXRT} \
--metadata-file ${METADATA} \
--req-fields-file ${REQ_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-07-2022 \
#--log-level trace \
#--diagnostics-flag true
