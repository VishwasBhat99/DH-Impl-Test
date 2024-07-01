use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct TradeFieldNames {
    pub MarketType: String,
    pub Isin: String,
    pub IsinDescription: String,
    pub MatDate: String,
    pub DealDate: String,
    pub SettType: String,
    pub SettDate: String,
    pub TradeAmt: String,
    pub TradePrice: String,
    pub TradeYield: String,
    pub Wap: String,
    pub Way: String,
    pub DealTime: String,
}

impl PartialOrd for TradeFieldNames {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.DealDate == other.DealDate {
            return Some(self.DealTime.cmp(&other.DealTime));
        }

        Some(self.DealDate.cmp(&other.DealDate))
    }
}
