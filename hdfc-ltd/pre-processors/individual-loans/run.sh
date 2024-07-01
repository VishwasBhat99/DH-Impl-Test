#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
MASTER_FILE=$"test-bed/input.txt"
excess_output_file=$"test-bed/output1.txt"
overdue_output_file=$"test-bed/output2.txt"
singleEIStructure_output_file=$"test-bed/output3.txt"
multipleEIStructure_output_file=$"test-bed/output4.txt"
multipleEIStructure1_output_file=$"test-bed/output5.txt"
multipleEIStructure2_output_file=$"test-bed/output6.txt"
multipleEIStructure3_output_file=$"test-bed/output7.txt"
PROD_GL_MASTER=$"test-bed/prod-gl-mapper.xlsx"

cargo run --release -- \
--input-master-file ${MASTER_FILE} \
--excess-output ${excess_output_file} \
--overdue-output ${overdue_output_file} \
--singleEIStructure-output ${singleEIStructure_output_file} \
--excesseEMIMultiple-output ${multipleEIStructure_output_file} \
--overdueEMIMultiple-output ${multipleEIStructure1_output_file} \
--multipleEMIAccounts-output ${multipleEIStructure2_output_file} \
--multiplePMIAccounts-output ${multipleEIStructure3_output_file} \
--as-on-date  31-05-2020 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--prod-gl-master-file ${PROD_GL_MASTER} \
--default-gls [1000,1000,1000,1000,1000] \
--prod-gl-master-sheet Sheet2 \
#--log-level trace \
#--diagnostics-flag false
