#!/usr/bin/env bash

$CSB_ROOT/close-stock/DB-Loader/Import/ImportData.sh

$CSB_ROOT/close-stock/DB-Loader/Export/ExportQuery.sh

echo "Security Closing Stock File Exported Successfully."
