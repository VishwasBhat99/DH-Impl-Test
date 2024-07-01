#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
INPUT_FILE=$"test-bed/Bond_FRA_Sample.txt"
OUTPUT_FILE=$"test-bed/output"
REF_FILE=$"test-bed/Derivatives_ContractTypology.xlsx"

cargo run -- \
--input-file ${INPUT_FILE} \
--output-file ${OUTPUT_FILE} \
--ref-file ${REF_FILE} \
--as-on-date  31-05-2020 \
--entity IND_CE \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} 
#--log-level trace \
#--diagnostics-flag false
