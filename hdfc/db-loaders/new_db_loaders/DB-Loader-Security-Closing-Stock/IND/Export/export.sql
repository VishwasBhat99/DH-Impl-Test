set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL &1/IND/&2/Sec_Close_Stock.txt;
select Bond_Issuance
||'|'||ISIN
||'|'||ins_dt
||'|'||Branch_ENTITY
||'|'||Desk
||'|'||Portfolio_Type
||'|'||Category
||'|'||Security_Type
||'|'||SLRNon_SLR
||'|'||Short_Name
||'|'||SecuredUnsecured
||'|'||Rate
||'|'||Nxt_Call_Dt
||'|'||Nxt_Put_Dt
||'|'||Agency
||'|'||Rating
||'|'||Agency_of_current_rating
||'|'||Listed_unlisted
||'|'||Mat_Dt
||'|'||Conversion_Rate_LCY
||'|'||Currency
||'|'||abs(BV_after_Amortisation)
||'|'||WAP
||'|'||LAF_and_MSF_Ost_FV
||'|'||LAF_and_MSF_Ost_BV
||'|'||Rev_LAF_Ost_FV
||'|'||Rev_Repo_Ost_FV
||'|'||Collateral_Placed_FV
||'|'||Encumbered_FV
||'|'||Encumbered_BV
||'|'||YTM
||'|'||Basis
||'|'||Issue_Country
||'|'||Domicile_Country
||'|'||Category1
||'|'||Category2
||'|'||Category3
||'|'||Category4
||'|'||Industry_code
||'|'||Taxability
||'|'||AIR_till_Date
||'|'||Modified_Duration
||'|'||Int_CouponType
||'|'||Nxt_Rep_Dt
||'|'||Sec_Grp
||'|'||Sec_Typ
||'|'||Sec_issuer
||'|'||Sec_guaranteed
||'|'||Mrkt
||'|'||Idx_label
||'|'||BD_CAT
||'|'||BD_TYP
||'|'||LSTD
||'|'||NPA
||'|'||cf_date
||'|'||abs(cf_int_amt)
||'|'||abs(cf_prin_amt)
||'|'||bv_before_amortisation
||'|'||facevalue
from Vw_Sec_Close_Stock
where Branch_ENTITY= 'INDIA_CE'
and BOND_ISSUANCE='Yes' ;
SPOOL OFF;
set termout on
