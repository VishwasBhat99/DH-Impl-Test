set verify off

insert  into "BALLG_MBTotals" (
"LLGID",
"AsOn",
"CurrencyID",
"DIM1",
"DIM2",
"DIM3",
"DIM4",
"DIM5",
"Amount",
"IR"
)
select
b."BALLGID",
to_date(t."AsOnDt",'DD-MM-YYYY') as "AsOnDt",
t."CcyID",
'DIM1',
'DIM2',
'DIM3',
'DIM4',
'DIM5',
Sum(t."Amount") as "Amount",NVL(Sum("InterestRate"*"Amount")/NULLIF(SUM("Amount"),0),0) as "AvgIR" from 
(select "BALLGID", max("EffFromDt") as "MaxEffDt" from "BALLGtoBALMLLGMap" where "EffFromDt" <= TO_DATE('&2','DD-MM-YYYY') group by "BALLGID") a,
 "BALLGtoBALMLLGMap" b, "BALMInputTotals" t where a."BALLGID"=b."BALLGID" and a."MaxEffDt"=b."EffFromDt" 
 and b."BALMLLGID"=t."BALMLLGID" and t."AsOnDt"='&2' and t."SLRorIRS" in ('SLR','ALL') and t."CountryID"=b."BALMCountryID" 
 group by b."BALLGID",t."CcyID",t."AsOnDt";
