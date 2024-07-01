#!/usr/bin/env bash

              
BALM_RULE_FILE="test-bed/ca_balm_rules.txt"       
BASEL_RULE_FILE="test-bed/ca_basel_rules.txt"     
BUCKET_SCHEME_FILE="test-bed/bucket_scheme.txt"                       
DIAGNOSTICS_LOG_FILE="test-bed/diag-log.txt"   
INPUT_FILE="test-bed/cf_ca_finacle_output_sample.cf"                
LOG_FILE="test-bed/log.txt"                    
METADATA_FILE="test-bed/casa_metadata.json"           
OUTPUT_FILE="test-bed/ca_output.txt"              
REQ_FIELDS_FILE="test-bed/req_fields.json"       
RULES_SCHEME_FILE="test-bed/rules_for_schemes.txt"                    
TBL_COMP_FILE="test-bed/tbl_computation.txt"    
EXCH="test-bed/exch.txt"

cargo run -- \
--as-on-date 31-05-2023 \
--balm-rule-file $BALM_RULE_FILE \
--basel-rule-file $BASEL_RULE_FILE \
--bucket-scheme-file $BUCKET_SCHEME_FILE \
--country IND \
--base-currency INR \
--default-balm-llg 1000 \
--default-basel-llg 9999 \
--diagnostics-log-file $DIAGNOSTICS_LOG_FILE \
--input-file $INPUT_FILE \
--log-file $LOG_FILE \
--metadata-file $METADATA_FILE \
--output-file $OUTPUT_FILE \
--req-fields-file $REQ_FIELDS_FILE \
--rules-scheme-file $RULES_SCHEME_FILE \
--scheme-id "99" \
--src-name "CA" \
--is-tbl-def-req "false" \
--is-aggr-on-basel-llg "false" \
--tbl-comp-file $TBL_COMP_FILE \
--log-level debug \
--exchange-rate-file $EXCH \