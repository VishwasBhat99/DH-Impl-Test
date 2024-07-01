#!/usr/bin/env bash

MOC=$"test-bed/GL_MOC_30112019.xlsx"
OUT=$"test-bed/output.txt"
CON=$"test-bed/concat.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_LLG=$"test-bed/MappingMaster1.xlsx"

cargo run --release -- \
--moc-input-file ${MOC} \
--output-file ${OUT} \
--concat-file ${CON} \
--alm-master ${MASTER_LLG} \
--alm-master-sheet-name Sheet1 \
--moc-sheet-name Sheet1 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
