#!/usr/bin/env bash

OUTPUT=$"test-bed/output18012023/output.txt"
LOG_FILE=$"test-bed/output18012023/log.txt"
DIAGNOSTICS_FILE=$"test-bed/output18012023/diaglog.txt"
INPUT_FILE=$"test-bed/input/TD-162_new.txt"
EDW_FILE=$"test-bed/master/edw.csv"
ORA_GL=$"test-bed/master/ORA_GL.xlsx"
MIS_DESC=$"test-bed/master/MIS1_Desc.xlsx"
LLG_MASTER=$"test-bed/master/Master_LLG_Updated.xlsx"
TENOR_DESC=$"test-bed/master/ORA_GL.xlsx"
CUST_CAT=$"test-bed/master/categ_master.xlsx"
LCR_MASTER=$"test-bed/master/lcr_master.txt"
WD_NWD=$"test-bed/master/categ_master.xlsx"


cargo run -- \
--output-file ${OUTPUT} \
--log-file-path ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 19-11-2020 \
--input-file ${INPUT_FILE} \
--edw-alm-td-file ${EDW_FILE} \
--ora-gl-master ${ORA_GL} \
--ora-gl-sheet-name "Sheet1" \
--mis-desc-file ${MIS_DESC} \
--mis-sheet-name "Sheet1" \
--llg-master-file ${LLG_MASTER} \
--llg-sheet-name "Sheet1" \
--tenor-desc-file ${TENOR_DESC} \
--tenor-sheet-name "Sheet1" \
--cust-cat-master ${CUST_CAT} \
--cust-sheet-name "Sheet1" \
--lcr-master-name ${LCR_MASTER} \
--wd-nwd-master ${WD_NWD} \
--wd-nwd-sheet "Sheet1" 

#--log-level trace \
#--diagnostics-flag true
