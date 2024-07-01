#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Exchange-Rate/GC/delete.sh $2
/home/dbuser/programs/DB-Loader-Exchange-Rate/GC/commit.sh 
/home/dbuser/programs/DB-Loader-Exchange-Rate/GC/ImportData.sh $1 

/home/dbuser/programs/DB-Loader-Exchange-Rate/GC/commit.sh 


