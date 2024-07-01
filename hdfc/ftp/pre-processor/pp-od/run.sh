#!/usr/bin/env bash

INPUT=$"test-bed/i1.txt"
OUTPUT=$"test-bed/output"
AMB=$"test-bed/amb.txt"
CORE=$"test-bed/od_stamper_2.xlsx"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
WEAKER_MASTER=$"test-bed/od_stamper_2.xlsx"
EWS_MASTER=$"test-bed/od_stamper_2.xlsx"
ORA_GL_MASTER=$"test-bed/od_stamper_2.xlsx"
MIS_DESC=$"test-bed/od_stamper_2.xlsx"
BDP_COA=$"test-bed/od_stamper_2.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--amb-file ${AMB} \
--core-master ${CORE} \
--core-master-sheet-name Sheet \
--mis1-desc ${MIS_DESC} \
--bdp-coa-file ${BDP_COA} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--weaker-sec-master ${WEAKER_MASTER} \
--ews-weaker-master ${EWS_MASTER} \
--ora-gl-master ${ORA_GL_MASTER} \
--weaker-sec-sheet-name "Sheet2" \
--mis1-sheet-name "Sheet1" \
--bdp-coa-sheet-name "Sheet3" \
--ews-weaker-sheet-name "Sheet4" 
#--log-level trace \
#-diagnostics-flag true
