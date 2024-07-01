master=$( ls  /SH_INPUTDATA/IND/$1/master_opt.csv )

sqlldr $CON_STR_IND data=$master control=/home/dbuser/programs/DB-Loader-Option-Register/Import/OPTION_MASTER.ctl LOG=/home/dbuser/logs/IND/$1/OPTION_MASTER.log BAD=/home/dbuser/logs/IND/$1/OPTION_MASTER.BAD 
