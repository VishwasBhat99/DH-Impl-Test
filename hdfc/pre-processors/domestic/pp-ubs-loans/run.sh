#!/usr/bin/env bash

INPUT=$"test-bed/ubs_test_2acc.txt"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/UBS_Rate_Code_Master_ddmmyyyy.xlsx"
REF3=$"test-bed/Ora_GL.xlsx"
REF4=$"test-bed/Master_LLG_060322019.xlsx"
REF5=$"test-bed/Ora_PROD.xlsx"
REF6=$"test-bed/Spread_Org.xlsx"
REF7=$"test-bed/NPA_Master.txt"
REF8=$"test-bed/MIS2_Master.xlsx"
REF9=$"test-bed/Concat_YieldGrp_WR.xlsx"
REF10=$"test-bed/Master_UBS_COA.xlsx"
OUTPUT=$"test-bed/output/output.txt"
REC_OUTPUT=$"test-bed/UBSReconRpt.txt"
CONCAT=$"test-bed/concat.txt"
CUST_TYPES=$"test-bed/cust.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"
WEAKER_MASTER=$"test-bed/Weaker_sec_master.xlsx"
EWS_MASTER=$"test-bed/EWS_weaker_master.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref-file-5 ${REF5} \
--ref-file-6 ${REF6} \
--ref-file-7 ${REF7} \
--ref-file-8 ${REF8} \
--ref-file-9 ${REF9} \
--ref-file-10 ${REF10} \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${REC_OUTPUT} \
--log-file ${LOG_FILE} \
--alm-sheet-name "Master" \
--cust-type-file-path ${CUST_TYPES} \
--weaker-sec-master ${WEAKER_MASTER} \
--ews-weaker-master ${EWS_MASTER} \
--weaker-sec-sheet-name "Sheet1" \
--ews-weaker-sheet-name "Sheet1" \
--mis2-master-sheet-name "Sheet1" \
--concat-yield-grp-sheet-name "Sheet1" \
--master-ubs-coa-sheet-name "Sheet1" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--sma-file ${SMA-FILE} \
--data-source-name ubs-loans \
--log-level debug \
# --diagnostics-flag true
