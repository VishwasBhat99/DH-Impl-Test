#!/usr/bin/env bash

FILENAME=$SH_SUMMARY/$3/BA/$1/balm_data.txt

sqlldr $CON_STR_BAUSR \
control=$SCRIPTS/$3/loader-scripts/BA/CTL/load_balm_data.ctl \
data=$FILENAME \
log=$SH_LOGS/$3/BA/$1/balm-data-loader.log \
bad=$SH_LOGS/$3/BA/$1/balm-data-loader.bad

$SCRIPTS/$3/loader-scripts/BA/commit.sh
