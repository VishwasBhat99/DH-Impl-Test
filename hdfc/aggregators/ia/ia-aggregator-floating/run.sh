#!/usr/bin/env bash

INPUT=$"test-bed/nCFOutput.cf"
REF1=$"test-bed/Spread_ref_win.xlsx"
OUTPUT=$"test-bed/output/aggregated"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
REQ_FIELDS_FILE=$"test-bed/dep-req-fields.json"
METADATA_FILE=$"test-bed/metadata-finnware-casa-od.json"
RULES_FILE=$"test-bed/dep_rules.txt"
TENOR_FILE=$"test-bed/tenor.txt"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file ${REF1} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--tenor-file ${TENOR_FILE} \
--src-local-ccy INR \
--display-local-ccy RUP \
--consol-ccy INR \
--foreign-consol-ccy FCY \
--exchange-rate-file $CURRENCY_CONV_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--account-metadata-file $METADATA_FILE \
--rules-file-path ${RULES_FILE} \
--as-on-date 01-02-2019 \
--account-level-exchange-rate false \
--default-llg-code 08888 \
--npa-flag-values "DOUBTFUL I,Doubtful II,LOSS,DOUBTFUL III,SUBSTANDARD " \
--tenor-flag false \
--is-consolidated false 
#--log-level trace \
#--diagnostics-flag true
