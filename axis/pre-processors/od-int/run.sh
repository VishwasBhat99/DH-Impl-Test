#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/GAM.txt"
ICV=$"test-bed/ICV.txt"
ITC=$"test-bed/ITC.txt"
IVS=$"test-bed/IVS.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run -- \
--input-file-gam ${GAM} \
--input-file-icv ${ICV} \
--input-file-itc ${ITC} \
--input-file-ivs ${IVS} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
