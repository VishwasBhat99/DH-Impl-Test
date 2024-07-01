#!/usr/bin/env bash

GAM=$"test-bed/gam.txt"
GAC=$"test-bed/gac.txt"
CMR=$"test-bed/cmg.txt"
EXRT=$"test-bed/exrt.txt"
RCT=$"test-bed/rct.txt"
ITC=$"test-bed/itc.txt"
EAB=$"test-bed/eab.txt"
GSP=$"test-bed/gsp.txt"
OUTPUT=$"test-bed/output.txt"
LLGMAP=$"test-bed/src.txt"
MASTER=$"test-bed/master.txt"
META=$"test-bed/meta.json"
RULES=$"test-bed/rules.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--gam-file-path ${GAM} \
--gac-file-path ${GAC} \
--cmg-file-path ${CMR} \
--ex-rt-file-path ${EXRT} \
--itc-file-path ${ITC} \
--eab-file-path ${EAB} \
--gsp-file-path ${GSP} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--mapping-master-file ${MASTER} \
--as-on-date 27-09-2021 \
--log-level trace \
--diagnostics-flag true
