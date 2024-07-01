#!/usr/bin/env bash

MASTER_FILE=$"test-bed/loans_master.txt"
REF_1_FILE=$"test-bed/foreclosure_rate.txt"
REF_2_FILE=$"test-bed/fixed_floating_rate.txt"
REF_3_FILE=$"test-bed/benchmark.txt"
REF_4_FILE=$"test-bed/ref4.xlsx"
REF_5_FILE=$"test-bed/ref5.txt"
CUST_ENTITY_MASTER=$"test-bed/cust-entity-master.txt"
CRM_MASTER=$"test-bed/crm-master.txt"
OUTPUT_FILE=$"test-bed/pp-out.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--master-file ${MASTER_FILE} \
--cust-entity-master-file ${CUST_ENTITY_MASTER} \
--crm-master-file ${CRM_MASTER} \
--ref1-file ${REF_1_FILE} \
--ref1-sheet-name "Sheet1" \
--ref2-file ${REF_2_FILE} \
--ref3-file ${REF_3_FILE} \
--ref4-file ${REF_4_FILE} \
--ref4-sheet-name "Sheet1" \
--ref5-file ${REF_5_FILE} \
--output-file ${OUTPUT_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 28-02-2023 \
--log-level trace \
--diagnostics-flag true \
