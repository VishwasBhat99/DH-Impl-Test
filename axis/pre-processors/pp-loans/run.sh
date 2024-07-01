#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
GAM=$"test-bed/gam_sample.txt"
ICV=$"test-bed/ICV.txt"
ITC=$"test-bed/ITC.txt"
IVS=$"test-bed/IVS.txt"
NPA=$"test-bed/NPA.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--diagnostics-log-file $DIAGNOSTICS_FILE \
--finacle-finlite-flg "FINACLE" \
--input-file-benchmark test-bed/LOAN/pp_od_benchmark_laa_output.txt \
--input-file-gam test-bed/LOAN/pp_gam_output_finacle_LAA.txt \
--input-file-int-rate test-bed/LOAN/pp_loan_int_finacle_output.txt \
--input-file-lam test-bed/LOAN/LAM_FINACLE.txt \
--input-file-lrs test-bed/LOAN/LRS_FINACLE.txt \
--input-file-lsp test-bed/LOAN/LSP_FINACLE.txt \
--input-file-od-int test-bed/LOAN/pp_od_int_finacle_output.txt \
--input-file-npa test-bed/LOAN/pp_npa_finacle.txt \
--log-file $LOG_FILE \
--output-file test-bed/LOAN/loan_finacle_uat_output.txt \
--log-level debug \
#--diagnostics-flag false
