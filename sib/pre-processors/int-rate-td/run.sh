#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/GAM.TXT"
RHT=$"test-bed/RHT.TXT"
TVS=$"test-bed/TVS.TXT"
ICV=$"test-bed/ICV.TXT"
ITC=$"test-bed/ITC.TXT"
TAM=$"test-bed/TAM.TXT"
OUTPUT_FILE=$"test-bed/output.txt"
LRP_FLT=$"test-bed/lrp_flt.txt"

cargo run -- \
--gam-file-path ${GAM} \
--itc-file-path ${ITC} \
--icv-file-path ${ICV} \
--rht-file-path ${RHT} \
--tvs-file-path ${TVS} \
--tam-file-path ${TAM} \
--output-file ${OUTPUT_FILE} \
--as-on-date 30-06-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
