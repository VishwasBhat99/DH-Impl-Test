#!/usr/bin/env bash

INPUT=$"test-bed/tb/OD_Master.txt"
OUTPUT=$"test-bed/tb/pp-out-overdraft.txt"
REF1=$"test-bed/tb/Asset_MCLR_Details.txt"
REF2=$"test-bed/tb/NPA_CLASSIFICATION_1.txt"
REF3=$"test-bed/tb/NPA_CLASSIFICATION_2.txt"
TA_CONFIG=$"test-bed/tb/TA_config.txt"
DLOD1=$"test-bed/tb/dlod1.txt"
DLOD2=$"test-bed/tb/dlod2.txt"
ODFD=$"test-bed/tb/odfd.txt"
RTL=$"test-bed/tb/rtl.txt"
LOG_FILE=$"test-bed/tb/log.txt"
DIAGNOSTICS_FILE=$"test-bed/tb/diag-log.txt"
CUST_ENTITY_MASTER=$"test-bed/cust-entity-master.txt"
CRM_MASTER=$"test-bed/crm-master.txt"


cargo run -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--reference-file-1 ${REF1} \
--reference-file-2 ${REF2} \
--reference-file-3 ${REF3} \
--dlod-cashflow-file ${DLOD1} \
--dlod-cashflow-file-2 ${DLOD2} \
--odfd-cashflow-file ${ODFD} \
--rtl-cashflow-file ${RTL} \
--ta-config-file ${TA_CONFIG} \
--asset-type "CC/OD/DLOD" \
--dlod-date-format dd-mm-yyyy \
--dlod-2-date-format dd-mm-yyyy \
--rtl-date-format dd-mmm-yyyy \
--as-on-date 30-06-2023 \
--dlod-2-delim "|" \
--dlod-delim "," \
--odfd-delim "|" \
--cust-entity-master-file ${CUST_ENTITY_MASTER} \
--crm-master-file ${CRM_MASTER} \
--reference-1-delim "," \
--reference-2-delim "|" \
--reference-3-delim "|" \
--rtl-delim "|" \
--ta-config-delim "|" \
--crm-file-delim "~#~" \
--cust-entity-delim "~#~"
