#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/pp_gam_output_finacle_TDA1.txt"
TAM=$"test-bed/BALM_TAM1.txt"
TVS=$"test-bed/BALM_TVS2.txt"
ICV=$"test-bed/BALM_ICV2.txt"
ITC=$"test-bed/pp_itc_output_finacle_ITC_DEFAULT1.txt"
OUTPUT_FILE=$"test-bed/pp_td_int_output.txt"

cargo run -- \
--input-file-gam ${GAM} \
--input-file-icv ${ICV} \
--input-file-tam ${TAM} \
--input-file-itc ${ITC} \
--input-file-tvs ${TVS} \
--output-file ${OUTPUT_FILE} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level debug \
--diagnostics-flag true
