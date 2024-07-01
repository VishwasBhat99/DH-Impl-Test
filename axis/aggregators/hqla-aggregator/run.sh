#!/usr/bin/env bash

INPUT_SEC_ALL=$"test-bed/HQLA-new/SecuritiesALL.lst"
INPUT_SEC_CBLO=$"test-bed/HQLA-new/SecuritiesCBLO.lst"
INPUT_SEC_CCIL=$"test-bed/HQLA-new/SecuritiesCCIL.lst"
INPUT_SEC_REPO=$"test-bed/HQLA-new/SecuritiesREPO.lst"
INPUT_MANUAL=$"test-bed/HQLA-new/ManualFile.TXT"
INPUT_HQLA=$"test-bed/HQLA-new/HQLAParameter.txt"
REQ_MANUAL=$"test-bed/req_man_fields.json"
OUTPUT=$"test-bed/HQLA-new/FinalSecuritiesAll.txt"
LOG_FILE=$"test-bed/HQLA-new/log.txt"
DIAGNOSTICS_FILE=$"test-bed/HQLA-new/diag-log.txt"

cargo run -- \
--input-sec-all-file ${INPUT_SEC_ALL} \
--input-sec-cblo-file ${INPUT_SEC_CBLO} \
--input-sec-ccil-file ${INPUT_SEC_CCIL} \
--input-sec-repo-file ${INPUT_SEC_REPO} \
--input-manual-file ${INPUT_MANUAL} \
--input-hqla-file ${INPUT_HQLA} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country-id "INDIA" \
--input-delimiter "|" \
--input-date-formats "%Y-%m-%d %H:%M:%S%.3f","%Y-%m-%d" \
--book-categories "HFT, FHFT, AFS, HTM" \
--required-manual-fields-file ${REQ_MANUAL} \
--as-on-date 05-04-2024 \
--accrued-day-convention "Accrued30/360" \
--default-repo-mat-date 01-01-1900 \
--diagnostics-flag true
