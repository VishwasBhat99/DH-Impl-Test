#!/usr/bin/env bash

CUR_MTH_SRC=$"test-bed/output.cf"
REQ_FIELDS_FILE=$"test-bed/ir_req_fields.json"
METADATA_FILE=$"test-bed/metadata.json"
SUMMARY=$"test-bed/summary.txt"
DRILLDOWN=$"test-bed/drilldown.txt"
MASTER=$"test-bed/master.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
LCR_RET=$"test-bed/lcr1.txt"
LCR_NON_RET=$"test-bed/lcr2.txt"


cargo run -- \
--cur-mth-src-file ${CUR_MTH_SRC} \
--req-fields-file-path $REQ_FIELDS_FILE \
--account-metadata-file-path $METADATA_FILE \
--source-system "Loans" \
--summary-file ${SUMMARY} \
--drilldown-file ${DRILLDOWN} \
--master-file ${MASTER} \
--sheet-name "Sheet1" \
--as-on-date 12-12-2020 \
--non-ret-cust-aggr-lcy-file ${LCR_NON_RET} \
--ret-cust-aggr-lcy-file ${LCR_RET} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true
