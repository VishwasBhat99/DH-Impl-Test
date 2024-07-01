
INPUT_FILE=$"test-bed/input.txt"
MASTER_FILE=$"test-bed/master.xlsx"
NPA_MASTER_FILE=$"test-bed/npa.csv"
OUTPUT_FILE=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
REP_DATE=$"test-bed/gl_master.xlsx"



cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT_FILE} \
--master-file ${MASTER_FILE} \
--npa-master-file ${NPA_MASTER_FILE} \
--npa-sheet-name "Sheet1" \
--output-file ${OUTPUT_FILE} \
--sheet-name "LLG Master" \
--date-fields "15|16|17|18|33|35" \
--header-rows "1" \
--as-on-date 27-09-2023 \
--rep-master-file ${REP_DATE} \
--rep-sheet-name Sheet1 
#--log-level trace \
#--diagnostics-flag true
