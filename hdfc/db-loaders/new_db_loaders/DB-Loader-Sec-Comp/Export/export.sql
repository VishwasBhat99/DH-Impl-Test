set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $INPUT/IND/&1/murex-sec-comp.txt
select A2.Dealno
||'|'||A2.Instrid
||'|'||A2.ShortName
||'|'||A2.IntrTyp
||'|'||A2.IntrAppFreq
||'|'||A2.CompoundFreq
||'|'||A2.IntrPrac
||'|'||A2.Coupon
||'|'||A2.LastIntrdt
||'|'||A2.NextIntrdt
||'|'||(CASE WHEN M1.NXT_REP_DT IS NOT NULL THEN
to_char(M1.NXT_REP_DT,'DD-MM-')||'20'||substr(to_char(M1.NXT_REP_DT,'YYYY'),3)
ELSE
A2.Matdt
END)
||'|'||A2.RateSpread
||'|'||A2.Rating
||'|'||A2.Matdt
||'|'||A2.calldate
||'|'||A2.putdate
||'|'||A2.Taxstatus
||'|'||A2.Product
||'|'||A2.ProductDesc
||'|'||A2.GLCode
||'|'||A2.Dealdt
||'|'||A2.Days_Trade_Date_Wise
||'|'||to_char(A2.ValueDt,'DD-MM-')||'20'||substr(to_char(A2.Valuedt,'YYYY'),3)
||'|'||A2.Days_Value_Date_Wise
||'|'||A2.Portfolio
||'|'||A2.PortfolioType
||'|'||A2.DealYTM
||'|'||A2.DealRt
||'|'||A2.AvgO_SVD
||'|'||A2.OrigFaceValue
||'|'||A2.O_SFaceValue
||'|'||A2.OrgCostvalue
||'|'||A2.O_SCostvalue_Before_Amort
||'|'||A2.Accruedint
||'|'||A2.InterestIncome
||'|'||A2.WAP_IGAAP
||'|'||A2.AmortTillDate
||'|'||A2.PeriodAmort
||'|'||A2.OSCostValue_After_Amort
||'|'||A2.BookYield
||'|'||A2.YieldonAvgOSVD
||'|'||A2.MarketValue
||'|'||A2.Security_Group
||'|'||A2.Security_Type
||'|'||A2.Security_issuer
||'|'||A2.Security_guaranteed
||'|'||A2.Market
||'|'||A2.Index_label
||'|'||A2.BD_CATEG
||'|'||A2.BD_Type
||'|'||A2.LISTED
||'|'||A2.NPA_CLASS
||'|'||A2.Entity
||'|'||A2.Desk
||'|'||A2.Accounting_Section_IGAAP
||'|'||nvl(replace(A1.VALUE_DATE,'/','-'),A2.Matdt)
||'|'||(CASE WHEN M1.NXT_REP_DT IS NOT NULL THEN
to_char(M1.NXT_REP_DT,'DD-MM-')||'20'||substr(to_char(M1.NXT_REP_DT,'YYYY'),3)
ELSE
A2.Matdt
END)--irr_DATE
----||'|'||nvl(M1.MATURITY,TO_DATE(A1.IRR_DATE,'DD-MM-YYYY'))
||'|'||nvl(sum(abs(A1.FLOWAMOUNT)),A2.OSCostValue_After_Amort)
||'|'||nvl(A1.FLOWTYPE,'CAP')
||'|'||A2.ISIN
from VW_SEC_COMP A2 
LEFT OUTER JOIN MUREX_SEC_COMP_CF A1 ON A2.DEALNO=A1.DEALNO
LEFT OUTER JOIN MUREX_SEC_COMP_MATT M1 ON A2.ISIN=M1.ISIN 
where A2.Entity = 'INDIA_CE' and 
A2.OSCostValue_After_Amort > 0
group by a1.flowtype 
,A2.Dealno
,A2.Instrid
,A2.ShortName
,A2.IntrTyp
,A2.IntrAppFreq
,A2.CompoundFreq
,A2.IntrPrac
,A2.Coupon
,A2.LastIntrdt
,A2.NextIntrdt
,(CASE WHEN M1.NXT_REP_DT IS NOT NULL THEN
to_char(M1.NXT_REP_DT,'DD-MM-')||'20'||substr(to_char(M1.NXT_REP_DT,'YYYY'),3)
ELSE
A2.Matdt
END)
,A2.RateSpread
,A2.Rating
,A2.Matdt
,A2.Dealdt
,A2.Dealdt
,A2.Taxstatus
,A2.Product
,A2.ProductDesc
,A2.GLCode
,A2.Dealdt
,A2.Days_Trade_Date_Wise
,A2.Valuedt
,A2.Days_Value_Date_Wise
,A2.Portfolio
,A2.PortfolioType
,A2.DealYTM
,A2.DealRt
,A2.AvgO_SVD
,A2.OrigFaceValue
,A2.O_SFaceValue
,A2.OrgCostvalue
,A2.O_SCostvalue_Before_Amort
,A2.Accruedint
,A2.InterestIncome
,A2.WAP_IGAAP
,A2.AmortTillDate
,A2.PeriodAmort
,A2.OSCostValue_After_Amort
,A2.BookYield
,A2.YieldonAvgOSVD
,A2.MarketValue
,A2.Security_Group
,A2.Security_Type
,A2.Security_issuer
,A2.Security_guaranteed
,A2.Market
,A2.Index_label
,A2.BD_CATEG
,A2.BD_Type
,A2.LISTED
,A2.NPA_CLASS
,A2.Entity
,A2.Desk
,A2.Accounting_Section_IGAAP
,A1.VALUE_DATE
,A1.IRR_DATE
,A2.ISIN
,A2.calldate
,A2.putdate
,M1.MATURITY
order by A2.DEALNO, a1.flowtype, a2.valuedt;
SPOOL OFF;
set termout on

