#!/usr/bin/env bash

GAM=$"test-bed/gam.txt"
GAC=$"test-bed/gac.txt"
CMR=$"test-bed/cmg.txt"
EXRT=$"test-bed/exrt.txt"
RCT=$"test-bed/rct.txt"
UCIF=$"test-bed/ucif.txt"
OUTPUT=$"test-bed/output_"
SRCMAP=$"test-bed/src.txt"
META=$"test-bed/meta.json"
RULES=$"test-bed/rules.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--gam-file-path ${GAM} \
--gac-file-path ${GAC} \
--cmg-file-path ${CMR} \
--ex-rt-file-path ${EXRT} \
--rct-file-path ${RCT} \
--ucif-file-path ${UCIF} \
--output-file ${OUTPUT} \
--srcmap-file ${SRCMAP} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-09-2021 \
--log-level trace \
--diagnostics-flag true
