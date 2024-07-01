
#!/usr/bin/env bash

INPUT=$"test-bed/input.txt"
OUT=$"test-bed/output.txt"
REC=$"test-bed/ReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MUREX=$"test-bed/Murex_Inv_Master.xlsx"
MASTER_LLG=$"test-bed/Master_LLG.xlsx"
ORA_GL=$"test-bed/Ora_GL.xlsx"
ENTITY=$"INDIA_CE"
CONCAT=$"test-bed/concat.txt"

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
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 29-01-2020 \
--log-level trace \
--diagnostics-flag true \
--entity ${ENTITY} \
--concat ${CONCAT} \
