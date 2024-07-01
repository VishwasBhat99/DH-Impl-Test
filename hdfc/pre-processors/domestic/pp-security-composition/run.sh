#!/usr/bin/env bash

INPUT=$"test-bed/sam.txt"
REF1=$"test-bed/MIS1_Desc.xlsx"
REF2=$"test-bed/Ora_GL.xlsx"
REF3=$"test-bed/Master_LLG.xlsx"
REF4=$"test-bed/Etrsry.xlsx"
REF5=$"test-bed/hft.xlsx"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/InvstReconRpt.txt"
CONCAT=$"test-bed/concat.txt"
LOG_FILE=$"test-bed/log.txt"
MUREX=$"test-bed/Murex_Inv_Master.xlsx"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
FB_MASTER=$"test-bed/floating_bond_master.csv"

cargo run --release -- \
--input-file ${INPUT} \
--ref-file-1 ${REF1} \
--ref-file-2 ${REF2} \
--ref-file-3 ${REF3} \
--ref-file-4 ${REF4} \
--ref-file-5 ${REF5} \
--alm-master-sheet-name "Master" \
--ref-file-5-sheet-name "Sheet1" \
--murex-inv-master ${MUREX} \
--murex-sheet-name Master \
--floating-bond-master ${FB_MASTER} \
--floating-bond-delimiter "|" \
--output-file ${OUTPUT} \
--concat-file ${CONCAT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-09-2019 \
--apply-defesance true \
--log-level trace \
--entity INDIA_CE \
--diagnostics-flag true
