#!/usr/bin/env bash

INPUT=$"test-bed/input/30092022/avg-daily-ca.txt"
PP_OUTPUT=$"test-bed/input/30092022/pp-out-ca.txt"
RECON=$"test-bed/recon.txt"
OUTPUT=$"test-bed/output/out.txt"
CLOSE=$"test-bed/output/close.txt"
LOG_FILE=$"test-bed/output/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--recon-file ${RECON} \
--avg-bal-input-file ${PP_OUTPUT} \
--avg-bal-pos 30 \
--output-file ${OUTPUT} \
--close-accounts-file ${CLOSE} \
--is-avgbal-absolute true \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-09-2022 \
--date-format DDMMYYYY \
--acr-int-amt-ccy-pos 30 \
--acr-int-amt-hcy-pos 31 \
--source CA
