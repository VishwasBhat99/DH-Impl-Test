#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/GAM.txt"
LAVS=$"test-bed/LAVS.txt"
LAM=$"test-bed/LAM.txt"
ICV=$"test-bed/ICV.txt"
ITC=$"test-bed/ITC.txt"
LRS=$"test-bed/LRS.txt"
OUTPUT_FILE=$"test-bed/output.txt"
LRP_FLT=$"test-bed/lrp_flt.txt"

cargo run -- \
--input-file-gam ${GAM} \
--input-file-icv ${ICV} \
--input-file-itc ${ITC} \
--input-file-lavs ${LAVS} \
--input-file-lam ${LAM} \
--input-file-lrs ${LRS} \
--output-file ${OUTPUT_FILE} \
--input-file-balm-lrp-flt ${LRP_FLT} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
