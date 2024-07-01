#!/usr/bin/env bash

BLRMS_FILE=$"test-bed/blrms_inp.csv"
NSLR_REPO=$"test-bed/nslr_repo.lst"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
EXCHANGE_RATE=$"test-bed/exchange_rate.txt"
COMMON_CODE=$"test-bed/commoncodes.txt"
BOND_MASTER=$"test-bed/bond.txt"

cargo run -- \
--blrms-file-path ${BLRMS_FILE} \
--nslr-file-path ${NSLR_REPO} \
--exchange-rate-file ${EXCHANGE_RATE} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--base-currency "INR" \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-level trace \
--diagnostics-flag true \
--common-code-file-path ${COMMON_CODE} \
--bond-master-file-path ${BOND_MASTER} \
--day-convention ACT30/360 \
--as-on-date 05-02-2024 
