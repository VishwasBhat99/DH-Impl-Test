#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUT=$"test-bed/output.txt"
REC=$"test-bed/ReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MUREX=$"test-bed/Murex_Inv_Master.xlsx"
MASTER_LLG=$"test-bed/Master_LLG_060322019.xlsx"
ISIN_MASTER=$"test-bed/isin_master.txt"
ORA_GL=$"test-bed/Ora_GL.xlsx"
ENTITY=$"INDIA_CE"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUT} \
--rec-output-file ${REC} \
--murex-inv-master ${MUREX} \
--ora-gl ${ORA_GL} \
--master-llg ${MASTER_LLG} \
--master-llg-sheet-name Master \
--ora-gl-sheet-name Sheet1 \
--murex-sheet-name Master \
--log-file ${LOG_FILE} \
--isin-master ${ISIN_MASTER} \
--as-on-date 30-04-2023 \
--diagnostics-log-file $DIAGNOSTICS_FILE \
--log-level trace \
--diagnostics-flag true \
--entity ${ENTITY} \
