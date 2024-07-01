#!/bin/bash
src_path=/nfs/SH_INPUTDATA/$1
dest_path=/nfs/SH_INPUTDATA
cd $src_path
find $src_path/* -type f -printf '%p :-> %t\n'> $dest_path/timestamp_$1.txt
