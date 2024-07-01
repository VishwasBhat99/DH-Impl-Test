#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
FBM_INPUT_FILE=$"test-bed/FBM.txt"
GAM_INPUT_FILE=$"test-bed/GAM.txt"
NPA_INPUT_FILE=$"test-bed/NPA.txt"
PP_FINACLE_BILLS_OUTPUT_FILE=$"test-bed/PP_Finacle_Bills_output.txt"


cargo run --release -- \
--fbm-input-file ${FBM_INPUT_FILE} \
--gam-input-file ${GAM_INPUT_FILE} \
--npa-input-file ${NPA_INPUT_FILE} \
--finacle-bills-output-file-path ${PP_FINACLE_BILLS_OUTPUT_FILE} \
--as-on-date  21-10-2022 \
--delimeter "|" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false