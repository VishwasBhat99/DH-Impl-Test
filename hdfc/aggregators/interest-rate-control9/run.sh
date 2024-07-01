#!/usr/bin/env bash

INPUT=$"test-bed/output.cf"
REQ_FIELDS_FILE=$"test-bed/td_req_fields.json"
METADATA_FILE=$"test-bed/td_metadata.json"
SUMMARY=$"test-bed/summary.txt"
DRILLDOWN=$"test-bed/drilldown.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
BUCKET=$"test-bed/bucket.xlsx"
CLUB_TEN_MASTER=$"test-bed/tenor.xlsx"
CUST_TYPE_MASTER=$"test-bed/cust_type.xlsx"
CLUB_TEN_RATE_MASTER=$"test-bed/tenor_rate.xlsx"
AMB_MASTER=$"test-bed/file.csv"
LCR_RET=$"test-bed/lcr1.txt"
LCR_NON_RET=$"test-bed/lcr2.txt"


cargo run -- \
--input-file-path ${INPUT} \
--req-fields-file-path ${REQ_FIELDS_FILE} \
--account-metadata-file-path ${METADATA_FILE} \
--summary-file ${SUMMARY} \
--drilldown-file ${DRILLDOWN} \
--club-ten-master ${CLUB_TEN_MASTER} \
--club-ten-rate-master ${CLUB_TEN_RATE_MASTER} \
--tenor-rate-senior "Senior Citizen" \
--tenor-rate-staff "Staff" \
--tenor-rate-others "Others" \
--cust-type-master ${CUST_TYPE_MASTER} \
--cust-type-senior-sheet "Senior Account" \
--cust-type-staff-sheet "Staff Account" \
--bucket-master ${BUCKET} \
--amb-master-file-path ${AMB_MASTER} \
--non-ret-cust-aggr-lcy-file ${LCR_NON_RET} \
--ret-cust-aggr-lcy-file ${LCR_RET} \
--as-on-date 30-10-2022 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag true
