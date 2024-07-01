CREATE TABLE "IND_BLR02A2"(
	"CountryCd" varchar2(16),
	"AsOnDt" char(10),
	"CurrencyId" varchar2(5),
	"CustGrpId" varchar2(24),
	"CustGrpName" varchar2(128),
	"SBAmtCcy" number(19, 3),
	"SBAmtHcy" number(19, 3),
	"CAAmtCcy" number(19, 3),
	"CAAmtHcy" number(19, 3),
	"TDAmtCcy" number(19, 3),
	"TDAmtHcy" number(19, 3),
 CONSTRAINT PK_IND_BLR02A2 PRIMARY KEY ("CountryCd","AsOnDt","CurrencyId","CustGrpId"));

