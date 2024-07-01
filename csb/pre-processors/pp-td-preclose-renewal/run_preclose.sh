#!/usr/bin/env bash

INPUT_preclose=$"test-bed/premat_cls-30062021.txt"
INPUT_renewal=$"test-bed/renewal_cls_30062021.txt"
OUTPUT=$"test-bed/TDprerenewPPop.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-preclose-file ${INPUT_preclose} \
--input-renewal-file ${INPUT_renewal} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true
