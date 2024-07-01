#!/usr/bin/env bash

INPUT=$"test-bed/ridf.txt"
OUT=$"test-bed/RIDFPreprocoutput.txt"
REC=$"test-bed/output/ReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/MappingMaster_v3_Updated-CRDR.xlsx"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--rec-output-file ${REC} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
