CREATE TABLE CCY_IRS_SWAP_MASTER
(
Entity VARCHAR2(128),
TradeId NUMERIC(24),
Contractid NUMERIC(24),
StructureIdLinkId NUMERIC(24),
ComponentTypology VARCHAR2(128),
PackageTypology VARCHAR2(128),
ContractTypology VARCHAR2(128),
ContractUsage VARCHAR2(128),
Desk VARCHAR2(128),
Book VARCHAR2(128),
Folder VARCHAR2(128),
TradingBanking VARCHAR2(128),
CounterpartyGroupCode VARCHAR2(128),
CounterpartyParentCode VARCHAR2(128),
CounterpartyChildCode VARCHAR2(128),
CounterpartyName VARCHAR2(128),
InternalExternal VARCHAR2(128),
TradeDate DATE NULL,
StartDate DATE NULL,
EndDate DATE NULL,
CurrencyPair VARCHAR2(128),
RecLegCurrency VARCHAR2(128),
OriginalNotionalRecLeg NUMBER(24,4),
OriginalNotionalRecLeginINR NUMBER(24,4),
OutstandingNotionalRecLeg NUMBER(24,4),
OutstandingNotionalRecLegInINR NUMBER(24,4),
PayLegCurrency VARCHAR2(128),
OriginalNotionalPayLeg NUMBER(24,4),
OriginalNotionalPayLegInINR NUMBER(24,4),
OutstandingNotionalPayLeg NUMBER(24,4),
OutstandingNotionalPayLeginINR NUMBER(24,4),
ContingentNotional NUMBER(24,4),
DealSide VARCHAR2(128),
PayLegIndex VARCHAR2(128),
PayIntRate NUMBER(24,4),
SpreadPayleg NUMBER(24,4),
RecLegIndex VARCHAR2(128),
RecIntRate NUMBER(24,4),
SpreadRecleg NUMBER(24,4),
PaySideAccuralInINR NUMBER(24,4),
PaySideMTMinINR NUMBER(24,4),
PaySideGMTMInINR NUMBER(24,4),
RecSideAccuralInINR NUMBER(24,4),
RecSideMTMInINR NUMBER(24,4),
RecSideGMTMInINR NUMBER(24,4),
NetAccrualinUSD NUMBER(24,4),
NetAccrualinINR NUMBER(24,4),
FutureCashProceedsCurrency VARCHAR2(128),
FutureCashProceeds VARCHAR2(128),
FutureCashProceedsINR NUMBER(24,4),
MarketValueFinanced NUMBER(24,4),
NetMTMinUSD NUMBER(24,4),
NetMTMinINR NUMBER(24,4),
NetGMTMinUSD NUMBER(24,4),
NetGMTMinINR NUMBER(24,4),
NetBCVAadjustedGMTMinINR NUMBER(24,4),
PaySidePV01InINR NUMBER(24,4),
RecSidePV01inINR NUMBER(24,4),
NetPV01inINR NUMBER(24,4),
PaySideModifiedDuration NUMBER(24,4),
ReceiveSideModifiedDuration NUMBER(24,4),
ModifiedDurationofthedeal NUMBER(24,4),
CVA NUMBER(24,4),
DVA NUMBER(24,4),
BCVA NUMBER(24,4),
PaylegExchangeRate NUMBER(24,4),
ReclegExchangeRate NUMBER(24,4),
PayResetDate DATE NULL,
RecResetDate DATE NULL,
PayPaymentDate DATE NULL,
RecPaymentDate DATE NULL,
BankOrNonBank VARCHAR2(128),
IndexRecLeg VARCHAR2(128),
IndexPayLeg VARCHAR2(128),
DayCountConventionRecleg VARCHAR2(128),
DayCountConventionPayLeg VARCHAR2(128),
OriginalTenor NUMBER(24,4),
ResidualTenor NUMBER(24,4),
PayResetFrequency VARCHAR2(128),
RecResetFrequency VARCHAR2(128),
PayPaymentFrequency VARCHAR2(128),
RecPaymentFrequency VARCHAR2(128),
UnderlyingPP VARCHAR2(128),
DealStatus VARCHAR2(128),
InputID VARCHAR2(128),
Comment1 VARCHAR2(128)
);
