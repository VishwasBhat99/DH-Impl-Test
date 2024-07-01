#!/usr/bin/env bash

INPUT=$"test-bed/edw-input.txt"
MASTER=$"test-bed/master.txt"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG_060322019.xlsx"
REF4=$"test-bed/FWCostCenter_OD.xlsx"
REF5=$"test-bed/INP001_NPA.txt"
REF6=$"test-bed/UBS_Rate_Code_Master_ddmmyyyy.xlsx"
REF7=$"test-bed/currency_master.xlsx"
REF8=$"test-bed/benchmark.txt"
REF9=$"test-bed/Spread_Org.xlsx"
REF10=$"test-bed/NPA_Master.txt"
OD_STUDY_MASTER=$"test-bed/od-study-master.xlsx"
OUTPUT=$"test-bed/new-finware-casaod.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/FWODReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--master-file ${MASTER} \
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
--od-study-master ${OD_STUDY_MASTER} \
--od-study-master-sheet-name "Shee1" \
--alm-master-sheet-name "Master" \
--benchmark-sheet-name "Sheet1" \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--concat-file ${CONCAT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
