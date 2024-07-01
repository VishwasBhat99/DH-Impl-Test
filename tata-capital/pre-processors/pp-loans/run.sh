#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
WRITEOFF=$"test-bed/writeoff.txt"
OUTPUT=$"test-bed/cf-ubs-loans-output.txt"
LOG_FILE=$"test-bed/cf-ubs-loans-log.txt"
CASHF=$"test-bed/cashflows.txt"
DIAGNOSTICS_FILE=$"test-bed/cf-ubs-loans-diag-log.txt"
TCFSL=$"test-bed/TCFSL.xlsx"

cargo run -- \
--finnone-fsl-file ${INPUT} \
--tcfsl-npa-file ${TCFSL} \
--writeoff-merged-file ${WRITEOFF} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--tcfsl-sheet-name "Sheet1" \
--finnnone-cashflow-file ${CASHF} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 28-02-2018 

