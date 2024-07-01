CREATE TABLE "IND_BLR02A11"(
	"CountryCd" varchar2(16),
	"AsOnDt" char(10),
	"CurrencyId" varchar2(5),
	"CustGrpId" varchar2(24),
	"CustGrpName" varchar2(128),
	"DepAmtCcy" number(19, 3),
	"DepAmtHcy" number(19, 3),
 CONSTRAINT PK_IND_BLR02A11 PRIMARY KEY ("CountryCd","AsOnDt","CurrencyId","CustGrpId"));
