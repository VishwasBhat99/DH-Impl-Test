#!/usr/bin/env bash

INPUT=$"test-bed/input.cf"
OUTPUT=$"test-bed/output.txt"
METADATA=$"test-bed/metadata.json"
REQ_FIELDS=$"test-bed/req_fields.json"
BALM_RULE=$"test-bed/rules.txt"
LOG=$"test-bed/log.txt"
DIAG_LOG=$"test-bed/diag-log.txt"
EX_RT=$"test-bed/1000ExchangeRate.txt"
CCY_FIELD=$"currency"

rm $OUTPUT

cargo run --release  -- \
--diagnostics-log-file $DIAG_LOG \
--input-file-path $INPUT \
--log-file $LOG \
--as-on-date 31-01-2021 \
--exchange-rate-file $EX_RT \
--base-currency OMR \
--acc-currency $CCY_FIELD \
--metadata-file-path $METADATA \
--output-file-path $OUTPUT \
--required-fields-file-path $REQ_FIELDS \
--balm-default-llg 5299 \
--balm-rule-file-path $BALM_RULE \
#--log-level trace \
#--diagnostics-flag false

