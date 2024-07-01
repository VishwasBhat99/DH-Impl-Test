#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BktData {
    pub from_bkt: i64,
    pub to_bkt: i64,
    pub bkt_id: String,
}
