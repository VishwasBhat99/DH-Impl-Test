#!/usr/bin/env bash

INPUT_FILE=$"test-bed/pp-out-gl.txt"
REF1=$"test-bed/alm_file_Master.xlsx"
REF3=$"test-bed/GLExcludeMaster.txt"
REF4=$"test-bed/Master.xlsx"
OUTPUT=$"test-bed/output"
CONCAT=$"test-bed/concat.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT_FILE} \
--ref-file-1 ${REF1} \
--gl-ex-master ${REF3} \
--gl-moc-file ${REF4} \
--gl-moc-sheet-name Sheet7 \
--alm-master-sheet-name Master \
--gl-moc-ccy INR \
--currency INR \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level trace \
--diagnostics-flag true
