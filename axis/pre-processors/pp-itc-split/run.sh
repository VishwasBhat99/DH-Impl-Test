#!/usr/bin/env bash

GAM=$"test-bed/input.txt"
GAC=$"test-bed/gac.txt"
CMR=$"test-bed/cmg.txt"
OUTPUT=$"test-bed/output_"
SRCMAP=$"test-bed/src.txt"
META=$"test-bed/meta.json"
RULES=$"test-bed/rules.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--gam-file-path ${GAM} \
--output-file ${OUTPUT} \
--srcmap-file ${SRCMAP} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-09-2021 \
--log-level trace \
--diagnostics-flag true
