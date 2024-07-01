#!/usr/bin/env bash

INPUT=$"testbed/CC_MAT_EXT1.txt"
OUTPUT=$"testbed/CFOutput"
RECON_OUT=$"testbed/Recon_CreditCard.txt"
LOG_FILE=$"testbed/log.txt"
DIAGNOSTICS_FILE=$"testbed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--rec-output-file ${RECON_OUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 01-01-2019 \
--log-level trace \
--diagnostics-flag true
