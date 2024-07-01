#!/usr/bin/env bash

INPUT_FILE=$"test-bed/CFOutput.cf"
COL_FILE=$"test-bed/col.txt"
OP_FILE=$"test-bed/Frwd-summary-rw"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag_log.txt"
METADATA=$"test-bed/metadata.json"
REQ=$"test-bed/req-fields.json"
RULES=$"test-bed/rules" 
EX_RATE=$"test-bed/1000ExchangeRate.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--default-sub-claim-id 10007777 \
--default-risk-weight 10000000 \
--ccy-mm-hc-prcnt 8.0 \
--exchange-rate-file ${EX_RATE} \
--mat-mm-hc-prcnt 9.0 \
--default-ccf-prcnt 50.0 \
--src-file-name "FRWD_CONTRACTS" \
--col-file-path ${COL_FILE} \
--input-file-path ${INPUT_FILE} \
--metadata-file-path ${METADATA} \
--rules-file-path ${RULES} \
--req-fields-file-path ${REQ} \
--base-currency INR \
--output-file-path ${OP_FILE} \
--as-on-date 30-06-2022 \
--log-level trace \
--diagnostics-flag false
