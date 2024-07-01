#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/GAM.txt"
IVS=$"test-bed/IVS.txt"
ICV=$"test-bed/ICV.txt"
ITC=$"test-bed/ITC.txt"
OUTPUT_FILE=$"test-bed/sba_int_output.txt"

cargo run -- \
--input-file-gam ${GAM} \
--input-file-icv ${ICV} \
--input-file-ivs ${IVS} \
--input-file-itc ${ITC} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
--diagnostics-flag true
