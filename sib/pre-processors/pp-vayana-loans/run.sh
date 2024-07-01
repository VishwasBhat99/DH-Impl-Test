#!/usr/bin/env bash
VAYANA="test-bed/vayana_loans.txt"
GAM="test-bed/GAM.TXT"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUT="test-bed/output.txt"
NPA=$"test-bed/npa.txt"

cargo run -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--gam-file-path ${GAM} \
--log-file ${LOG_FILE} \
--npa-file-path ${NPA} \
--output-file-path ${OUT} \
--vayana-loans-file-path ${VAYANA} \
