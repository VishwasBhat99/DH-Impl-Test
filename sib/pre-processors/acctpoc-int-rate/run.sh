#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/GAM.TXT"
LAVS=$"test-bed/LAVS.TXT"
IVS=$"test-bed/IVS.TXT"
ICV=$"test-bed/ICV.TXT"
PCA=$"test-bed/PCA.TXT"
ITC=$"test-bed/ITC.TXT"
TAM=$"test-bed/TAM.TXT"
OUTPUT_FILE=$"test-bed/output.txt"
LRP_FLT=$"test-bed/lrp_flt.txt"

cargo run -- \
--gam-file-path ${GAM} \
--itc-file-path ${ITC} \
--icv-file-path ${ICV} \
--pca-file-path ${PCA} \
--lavs-file-path ${LAVS} \
--ivs-file-path ${IVS} \
--output-file ${OUTPUT_FILE} \
--as-on-date 30-06-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false