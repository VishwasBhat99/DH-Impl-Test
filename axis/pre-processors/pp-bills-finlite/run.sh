#!/usr/bin/env bash

SCFM=$"test-bed/BALM_SCFM.TXT"
GL=$"test-bed/BALM_BILLS_GL.TXT"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--gl-file-path ${GL} \
--output-file ${OUTPUT} \
--scfm-file-path ${SCFM} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019
