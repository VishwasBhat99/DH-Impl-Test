#!/usr/bin/env bash
NPA_FILE=$"test-bed/gl_npa.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"
UBS_PATH=$"test-bed/ubs_master.txt"
CASH_FLOW_PATH=$"test-bed/cashflow.txt"


cargo run --release -- \
--npa-file-path ${NPA_FILE} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-10-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ubsloans-master-file ${UBS_PATH} \
--ubsloans-cashflow-file ${CASH_FLOW_PATH} \
--product-code-skip "" \
--cashflow-comp-skip "" \
#--log-level trace \
#--diagnostics-flag false