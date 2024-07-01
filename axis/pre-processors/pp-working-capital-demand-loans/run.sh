#!/usr/bin/env bash

BALM_GAM=$"test-bed/balm_gam.txt"
WCDL_DISB=$"test-bed/balm_wcdl.txt"
NPA_DATA=$"test-bed/npa_data.txt"
NPA_LIVE=$"test-bed/npa_live.txt"
NPA_CONFIG=$"test-bed/npa_config.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXCHANGE_RATE=$"test-bed/exrate.txt"
BALM_GAC=$"test-bed/balm_gac.txt"
BALM_RCT=$"test-bed/balm_rct.txt"

cargo run -- \
--balm-gam-file ${BALM_GAM} \
--wcdl-file-path ${WCDL_DISB} \
--exchange-rate-file ${EXCHANGE_RATE} \
--npa-config-file ${NPA_CONFIG} \
--npa-data-file ${NPA_DATA} \
--npa-live-file ${NPA_LIVE} \
--output-file ${OUTPUT} \
--currency INR \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--as-on-date 30-09-2023 \
--code-type "WCDL_Residual_days" \
--cm-code "UN30" \
--balm-gac-file ${BALM_GAC} \
--balm-rct-file ${BALM_RCT}
