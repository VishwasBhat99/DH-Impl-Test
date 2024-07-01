#!/usr/bin/env bash

CA=$"test-bed/uat/cf_ca_finacle_output.cf"
SA=$"test-bed/uat/sa.cf"
CASA_METADATA=$"test-bed/uat/casa_metadata.json"
OUTPUT=$"test-bed/uat/output.txt"
TD=$"test-bed/uat/td.cf"
LOG_FILE=$"test-bed/uat/log.txt"
DIAGNOSTICS_FILE=$"test-bed/uat/diaglog.txt"
RD=$"test-bed/uat/rd.cf"
TD_METADATA=$"test-bed/uat/td_metadata.json"
RD_METADATA=$"test-bed/uat/td_metadata.json"
CMG=$"test-bed/uat/cmg1.txt"
REQ_FIELDS_FILE=$"test-bed/domestic/req_fields.json"
SAL=$"test-bed/uat/SALARY_PENSION_RELN_DATA.txt"
STABLE_RELN=$"test-bed/uat/LCR_FILE_STABLE_RELN_DATA.txt"
CA_BALM_RULE_FILE=$"test-bed/uat/ca_finacle_rules.txt"
SA_BALM_RULE_FILE=$"test-bed/uat/sa_finacle_rules.txt"
TD_BALM_RULE_FILE=$"test-bed/uat/td_finacle_rules.txt"
RD_BALM_RULE_FILE=$"test-bed/uat/td_finacle_rules.txt"
NWD_FILE=$"test-bed/uat/nwd_file.txt"
TBL=$"test-bed/uat/tblDepositComputationDef.txt"

cargo run -- \
--ca-file ${CA} \
--ca-metadata-file ${CASA_METADATA} \
--cmg-file ${CMG} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file-path ${LOG_FILE} \
--rd-file ${RD} \
--rd-metadata-file ${RD_METADATA} \
--output-file ${OUTPUT} \
--req-fields-file ${REQ_FIELDS_FILE} \
--sa-file ${SA} \
--sa-metadata-file ${CASA_METADATA} \
--salary-pension-reln-file ${SAL} \
--stable-reln-file ${STABLE_RELN} \
--td-file ${TD} \
--td-metadata-file ${TD_METADATA} \
--as-on-date 31-05-2023 \
--ca-balm-rule-file ${CA_BALM_RULE_FILE} \
--rd-balm-rule-file ${RD_BALM_RULE_FILE} \
--sa-balm-rule-file ${SA_BALM_RULE_FILE} \
--td-balm-rule-file ${TD_BALM_RULE_FILE} \
--default-llg-code 1111 \
--is-nwd-code-in-use true \
--nwd-code-lookup ${NWD_FILE} \
--nwd-constitution-codes 2,3,4 \
--nwd-residual-days-limit 30 \
--tbl-dep-comp-def-file ${TBL}
#--log-level trace \
#--diagnostics-flag true

