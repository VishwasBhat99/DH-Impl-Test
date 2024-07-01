#!/usr/bin/env bash

INPUT=$"test-bed/pawan/TDAggregated"
OUTPUT=$"test-bed/pawan/TDAggregated"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
CURRENCY_CONV_FILE=$"test-bed/1000ExchangeRate.txt"
PRODUCT_RPT_FILE=$"test-bed/Book1.xlsx"
LLG_MAPPING_FILE=$"test-bed/llg_mapping.txt"
/home/pavan/Work/tempdir/SuperDB-Batch/hdfc/aggregators/balm/balm-smry-bif-mat/target/release/balm_smry_bif \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--product-rpt-file ${PRODUCT_RPT_FILE} \
--llg-mapping-file ${LLG_MAPPING_FILE} \
--exchange-rate-file $CURRENCY_CONV_FILE
#--log-level trace \
#--diagnostics-flag true
