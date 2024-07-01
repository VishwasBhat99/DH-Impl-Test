use dbpool::OracleConnectionManager;
use r2d2::Pool;

pub struct AccData {
    pub amount: f64,
    pub lcy_amount: f64,
    pub is_nwd_final: String,
    pub bucket_id: usize
}

// return is a tuple with amount, lcy_amount, is_nwd_final and bucket id values for a customer id from td db
pub fn get_bals_nwd_flg_td(
    cust_id: &String,
    pool: Pool<OracleConnectionManager>,
) -> Result<Vec<(AccData)>, String> {
    let conn = &pool.get().unwrap().conn;
    let mut cust_data: Vec<AccData> = Vec::new();
    match conn {
        Some(db) => {
            let sql = "select AMOUNT,LCY_AMOUNT,IS_NWD_FINAL,BKT_ID from TD_WD_RET where CUST_ID = :1";
            let rows = db
                .conn
                .query(sql, &[cust_id])
                .expect("Query Failed to Fetch Data from DB.");
            for row_result in &rows {
                let row = row_result.expect("Failed to read query output.");
                let amount: f64 = row.get("AMOUNT").unwrap_or(0.0);
                let lcy_amount: f64 = row.get("LCY_AMOUNT").unwrap_or(0.0);
                let is_nwd_final: String = row.get("IS_NWD_FINAL").unwrap_or("NONE".to_string());
                let bkt_id: usize = row.get("BKT_ID").unwrap_or(0);
                let acc_data = AccData {
                    amount: amount,
                    lcy_amount: lcy_amount,
                    is_nwd_final: is_nwd_final,
                    bucket_id: bkt_id
                };
                cust_data.push(acc_data);
            }
            return Ok(cust_data);
        }
        None => {
            return Err(format!(
                "Cannot not connect to oracle db. Please check credentials."
            ))
        }
    }
}

// return is a tuple with amount, lcy_amount, is_nwd_final and bucket id values for a customer id from td db
pub fn get_bals_nwd_flg_rd(
    cust_id: &String,
    pool: Pool<OracleConnectionManager>,
) -> Result<Vec<(AccData)>, String> {
    let conn = &pool.get().unwrap().conn;
    let mut cust_data: Vec<AccData> = Vec::new();
    match conn {
        Some(db) => {
            let sql = "select AMOUNT,LCY_AMOUNT,IS_NWD_FINAL,BKT_ID from RD_WD_RET where CUST_ID = :1";
            let rows = db
                .conn
                .query(sql, &[cust_id])
                .expect("Query Failed to Fetch Data from DB.");
            for row_result in &rows {
                let row = row_result.expect("Failed to read query output.");
                let amount: f64 = row.get("AMOUNT").unwrap_or(0.0);
                let lcy_amount: f64 = row.get("LCY_AMOUNT").unwrap_or(0.0);
                let is_nwd_final: String = row.get("IS_NWD_FINAL").unwrap_or("NONE".to_string());
                let bkt_id: usize = row.get("BKT_ID").unwrap_or(0);
                let acc_data = AccData {
                    amount: amount,
                    lcy_amount: lcy_amount,
                    is_nwd_final: is_nwd_final,
                    bucket_id: bkt_id
                };
                cust_data.push(acc_data);
            }
            return Ok(cust_data);
        }
        None => {
            return Err(format!(
                "Cannot not connect to oracle db. Please check credentials."
            ))
        }
    }
}
