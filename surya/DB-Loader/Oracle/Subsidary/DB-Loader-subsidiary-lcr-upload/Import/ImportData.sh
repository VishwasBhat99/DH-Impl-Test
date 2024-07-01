#!/usr/bin/env bash

sqlldr $CON_STR_BALMUSR \
data=../../../../../subsidary/subsidiary-lcr/test-bed/output.txt \
control=../subsidiary-lcr-upload.ctl \
LOG=../Logs/subsidiary-lcr-upload.log \
BAD=../Logs/subsidiary-lcr-upload.BAD
