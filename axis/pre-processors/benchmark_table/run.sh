#!/usr/bin/env bash
MCLR="test-bed/gam_mclr.txt"
GAM="test-bed/gam.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUT="test-bed/output.txt"

cargo run -- \
--balm-mclr ${MCLR} \
--gam-datafile ${GAM} \
--output-file-path ${OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag false
