#!/usr/bin/env bash

INR_INS_INPATH=$"test-bed/inr-irs.txt"
REF_PATH=$"test-bed/Derivatives_ContractTypology.xlsx"
INR_IRS_ND_PATH=$"test-bed/inr-irs-ND.txt"
INR_IRS_OUTPATH=$"test-bed/inr-irs-op.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/log.txt"

cargo run --release -- \
--inr-irs-infile ${INR_INS_INPATH} \
--ref-file ${REF_PATH} \
--inr-irs-nd-file ${INR_IRS_ND_PATH} \
--inr-irs-outfile ${INR_IRS_OUTPATH} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--sheet-name "Sheet1" \
--as-on-date "21-02-2022" \
--log-level trace \
--diagnostics-flag true 
