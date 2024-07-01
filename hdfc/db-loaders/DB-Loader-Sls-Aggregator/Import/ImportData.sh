sqlldr $CON_STR_BASEL_STR control=./IND_BLR02_Dep.ctl LOG=/data/oracle18c/app/product/18c/dbhome/ImportLog/IND_BLR02_Dep.log BAD=/data/oracle18c/app/product/18c/dbhome/ImportLog/IND_BLR02_Dep.BAD
sqlldr $CON_STR_BASEL_STR control=./IND_BLR02_Brw.ctl LOG=/data/oracle18c/app/product/18c/dbhome/ImportLog/IND_BLR02_Brw.log BAD=/data/oracle18c/app/product/18c/dbhome/ImportLog/IND_BLR02_Brw.BAD
sqlldr $CON_STR_BASEL_STR control=./IND_BLR02_Liab.ctl LOG=/data/oracle18c/app/product/16c/dbhome/ImportLog/IND_BLR02_Liab.log BAD=/data/oracle18c/app/product/18c/dbhome/ImportLog/IND_BLR02_Liab.BAD
