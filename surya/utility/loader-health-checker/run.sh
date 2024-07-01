#!/usr/bin/env bash
LOADER_LOG_FILE="test-bed/mssql-error-tata-capital.txt"
OUTPUT_FILE_PATH=$"test-bed/output.txt"
LOADER_FLAG="MSSQL"

cargo run --release --  \
-o ${OUTPUT_FILE_PATH} \
-l ${LOADER_LOG_FILE} \
-f ${LOADER_FLAG}
