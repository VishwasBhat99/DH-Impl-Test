#!/usr/bin/env bash

$CSB_ROOT/fx-spot/DB-Loader/Import/ImportData.sh

$CSB_ROOT/fx-spot/DB-Loader/Export/ExportQuery.sh

echo "FX Spot File Exported Successfully."
