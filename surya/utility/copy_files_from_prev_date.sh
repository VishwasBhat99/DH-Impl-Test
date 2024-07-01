#! /bin/bash

dateymd=$(busybox date -D %d%m%Y -d "$1" +%F)
prevdateymd=$(date -d "$dateymd - 1 days" +'%Y%m%d')
prevdate=$(date -d "$prevdateymd" +'%d%m%Y')
prevdir=/nfs/SH_INPUTDATA/$prevdate
currdir=/nfs/SH_INPUTDATA/$1
cp -R $prevdir/* $currdir/
find -type f $currdir/ -name '*'$prevdate'*' -delete
